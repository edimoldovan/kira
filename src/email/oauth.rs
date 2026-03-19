use crate::email::account::AccountConfig;
use oauth2::basic::BasicClient;
use oauth2::{
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope,
  TokenResponse, TokenUrl,
};
use async_std::io::WriteExt;
use async_std::net::TcpListener;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const REDIRECT_PORT: u16 = 8888;
const GMAIL_SCOPE: &str = "https://mail.google.com/";

pub struct XOAuth2 {
  response: Vec<u8>,
}

impl XOAuth2 {
  pub fn new(email: &str, access_token: &str) -> Self {
    let response = format!("user={}\x01auth=Bearer {}\x01\x01", email, access_token);
    Self {
      response: response.into_bytes(),
    }
  }
}

impl async_imap::Authenticator for XOAuth2 {
  type Response = Vec<u8>;

  fn process(&mut self, _challenge: &[u8]) -> Self::Response {
    self.response.clone()
  }
}

fn build_oauth_client(account: &AccountConfig) -> BasicClient {
  BasicClient::new(
    ClientId::new(account.client_id.clone()),
    Some(ClientSecret::new(account.client_secret.clone())),
    AuthUrl::new(GOOGLE_AUTH_URL.to_string()).unwrap(),
    Some(TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).unwrap()),
  )
  .set_redirect_uri(
    RedirectUrl::new(format!("http://127.0.0.1:{}", REDIRECT_PORT)).unwrap(),
  )
}

/// Run the full OAuth2 authorization flow: open browser, catch redirect, return tokens.
/// Returns (access_token, refresh_token).
pub async fn authorize(account: &AccountConfig) -> Result<(String, String), String> {
  let client = build_oauth_client(account);

  let (auth_url, _csrf) = client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new(GMAIL_SCOPE.to_string()))
    .add_extra_param("access_type", "offline")
    .add_extra_param("prompt", "consent")
    .url();

  open::that(auth_url.as_str()).map_err(|e| format!("Failed to open browser: {}", e))?;

  let code = listen_for_redirect().await?;

  let (access_token, refresh_token) = async_std::task::spawn_blocking(move || {
    let token_result = client
      .exchange_code(AuthorizationCode::new(code))
      .request(oauth2::reqwest::http_client)
      .map_err(|e| format!("Token exchange failed: {}", e))?;

    let access_token = token_result.access_token().secret().clone();
    let refresh_token = token_result
      .refresh_token()
      .map(|t| t.secret().clone())
      .ok_or_else(|| "No refresh token returned".to_string())?;

    Ok::<_, String>((access_token, refresh_token))
  })
  .await?;

  Ok((access_token, refresh_token))
}

/// Use a refresh token to get a new access token.
pub async fn refresh_access_token(account: &AccountConfig) -> Result<String, String> {
  let client = build_oauth_client(account);
  let refresh_token = account.refresh_token.clone();

  async_std::task::spawn_blocking(move || {
    let token_result = client
      .exchange_refresh_token(&RefreshToken::new(refresh_token))
      .request(oauth2::reqwest::http_client)
      .map_err(|e| format!("Token refresh failed: {}", e))?;

    Ok(token_result.access_token().secret().clone())
  })
  .await
}

/// Listen on localhost for the OAuth redirect and extract the authorization code.
async fn listen_for_redirect() -> Result<String, String> {
  let listener = TcpListener::bind(format!("127.0.0.1:{}", REDIRECT_PORT))
    .await
    .map_err(|e| format!("Failed to bind redirect listener: {}", e))?;

  let (mut stream, _) = listener
    .accept()
    .await
    .map_err(|e| format!("Failed to accept connection: {}", e))?;

  let mut buf = vec![0u8; 4096];
  let n = async_std::io::ReadExt::read(&mut stream, &mut buf)
    .await
    .map_err(|e| format!("Failed to read request: {}", e))?;

  let request = String::from_utf8_lossy(&buf[..n]);

  // Extract code from "GET /?code=XXXX&... HTTP/1.1"
  let code = request
    .split_whitespace()
    .nth(1)
    .and_then(|path| {
      form_urlencoded::parse(path.trim_start_matches("/?").as_bytes())
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
    })
    .ok_or_else(|| "No authorization code in redirect".to_string())?;

  let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h2>Authorization complete!</h2><p>You can close this tab.</p></body></html>";
  let _ = stream.write_all(response.as_bytes()).await;

  Ok(code)
}
