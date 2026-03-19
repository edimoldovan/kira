use crate::email::account::AccountConfig;
use crate::email::account::Message;
use crate::email::cache::{load_folder_cache, save_folder_cache};
use crate::email::oauth;
use async_imap::types::Flag;
use async_imap::Session;
use async_native_tls::{TlsConnector, TlsStream};
use async_std::net::TcpStream;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use mailparse::{dateparse, parse_mail};

type ImapSession = Session<TlsStream<TcpStream>>;

pub async fn sync_account(account: &AccountConfig) -> Result<(), String> {
  let account = account.clone();
  async_std::task::spawn(async move { sync_account_inner(&account).await }).await
}

async fn sync_account_inner(account: &AccountConfig) -> Result<(), String> {
  let mut session = connect_imap(account).await?;

  session
    .select("INBOX")
    .await
    .map_err(|e| format!("Failed to select INBOX: {}", e))?;

  let mut cache = load_folder_cache(&account.email, "inbox");

  let fetch_range = if cache.last_uid > 0 {
    format!("{}:*", cache.last_uid + 1)
  } else {
    let total = get_message_count(&mut session).await?;
    let start = if total > 100 { total - 99 } else { 1 };
    format!("{}:{}", start, total)
  };

  {
    let mut messages_stream = session
      .fetch(&fetch_range, "(UID FLAGS ENVELOPE)")
      .await
      .map_err(|e| format!("Failed to fetch messages: {}", e))?;

    let mut new_messages = Vec::new();
    let mut max_uid = cache.last_uid;

    while let Some(msg_result) = messages_stream.next().await {
      if let Ok(msg) = msg_result {
        let uid = msg.uid.unwrap_or(0);
        if uid == 0 {
          continue;
        }

        let envelope = msg.envelope().ok_or("No envelope")?;

        let from = envelope
          .from
          .as_ref()
          .and_then(|addrs| addrs.first())
          .and_then(|addr| {
            addr
              .name
              .as_ref()
              .map(|n| String::from_utf8_lossy(n).to_string())
              .or_else(|| {
                addr
                  .mailbox
                  .as_ref()
                  .map(|m| String::from_utf8_lossy(m).to_string())
              })
          })
          .unwrap_or_default();

        let subject = envelope
          .subject
          .as_ref()
          .map(|s| String::from_utf8_lossy(s).to_string())
          .unwrap_or_default();

        let date = envelope
          .date
          .as_ref()
          .and_then(|d| dateparse(&String::from_utf8_lossy(d)).ok())
          .unwrap_or_else(|| Utc::now().timestamp());

        let message = Message {
          uid,
          from,
          subject,
          preview: String::new(),
          body: None,
          date: DateTime::from_timestamp(date, 0).unwrap_or_else(|| Utc::now()),
          unread: !msg.flags().any(|f| matches!(f, Flag::Seen)),
        };
        new_messages.push(message);

        max_uid = max_uid.max(uid);
      }
    }

    cache.messages.extend(new_messages);
    cache.messages.sort_by(|a, b| b.date.cmp(&a.date));
    cache.messages.truncate(100);
    cache.last_uid = max_uid;
    cache.last_sync = Some(Utc::now());
  }

  save_folder_cache(&account.email, "inbox", &cache)?;

  session
    .logout()
    .await
    .map_err(|e| format!("Failed to logout: {}", e))?;

  Ok(())
}

async fn connect_imap(account: &AccountConfig) -> Result<ImapSession, String> {
  let addr = format!("{}:{}", account.imap_server, account.imap_port);
  let tcp = async_std::future::timeout(
    std::time::Duration::from_secs(10),
    TcpStream::connect(&addr),
  )
  .await
  .map_err(|_| format!("Connection to {} timed out", addr))?
  .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

  let tls = TlsConnector::new();
  let tls_stream = tls
    .connect(&account.imap_server, tcp)
    .await
    .map_err(|e| format!("TLS connection failed: {}", e))?;

  let mut client = async_imap::Client::new(tls_stream);
  let _greeting = client.read_response().await;

  if account.is_oauth() {
    let access_token = oauth::refresh_access_token(account).await?;
    let auth = oauth::XOAuth2::new(&account.email, &access_token);
    client
      .authenticate("XOAUTH2", auth)
      .await
      .map_err(|e| format!("XOAUTH2 login failed: {}", e.0))
  } else {
    client
      .login(&account.username, &account.password)
      .await
      .map_err(|e| format!("Login failed: {}", e.0))
  }
}

async fn get_message_count(session: &mut ImapSession) -> Result<u32, String> {
  let mailbox = session
    .examine("INBOX")
    .await
    .map_err(|e| format!("Failed to examine INBOX: {}", e))?;

  Ok(mailbox.exists)
}

pub async fn fetch_message_body(
  account: &AccountConfig,
  uid: u32,
) -> Result<String, String> {
  let account = account.clone();
  async_std::task::spawn(async move { fetch_message_body_inner(&account, uid).await }).await
}

async fn fetch_message_body_inner(account: &AccountConfig, uid: u32) -> Result<String, String> {
  let mut session = connect_imap(account).await?;

  session
    .select("INBOX")
    .await
    .map_err(|e| format!("Failed to select INBOX: {}", e))?;

  let body = {
    let mut messages_stream = session
      .uid_fetch(format!("{}", uid), "RFC822")
      .await
      .map_err(|e| format!("Failed to fetch message: {}", e))?;

    if let Some(msg_result) = messages_stream.next().await {
      let msg = msg_result.map_err(|e| format!("Failed to get message: {}", e))?;
      if let Some(body_bytes) = msg.body() {
        if let Ok(parsed) = parse_mail(body_bytes) {
          extract_body(&parsed)
        } else {
          String::new()
        }
      } else {
        String::new()
      }
    } else {
      String::new()
    }
  };

  session
    .logout()
    .await
    .map_err(|e| format!("Failed to logout: {}", e))?;

  Ok(body)
}

fn html_to_text(html: &str) -> String {
  let with_breaks = html
    .replace("<br>", "\n")
    .replace("<br/>", "\n")
    .replace("<br />", "\n")
    .replace("</p>", "\n\n")
    .replace("</div>", "\n")
    .replace("</h1>", "\n\n")
    .replace("</h2>", "\n\n")
    .replace("</h3>", "\n\n")
    .replace("</li>", "\n")
    .replace("</tr>", "\n");

  // Extract link URLs from <a> tags
  let mut result = String::new();
  let mut in_tag = false;
  let mut current_tag = String::new();
  let mut in_link = false;

  let mut chars = with_breaks.chars();
  while let Some(c) = chars.next() {
    match c {
      '<' => {
        in_tag = true;
        current_tag.clear();
      }
      '>' => {
        in_tag = false;
        // Check if it's a link tag
        if current_tag.starts_with("a ") || current_tag.starts_with("a\t") {
          in_link = true;
          // Extract href
          if let Some(href_start) = current_tag.find("href=\"") {
            let url_start = href_start + 6;
            if let Some(url_end) = current_tag[url_start..].find('"') {
              let url = &current_tag[url_start..url_start + url_end];
              result.push_str("\n[Link: ");
              result.push_str(url);
              result.push_str("]\n");
            }
          }
        } else if current_tag == "/a" {
          in_link = false;
        }
        current_tag.clear();
      }
      _ if in_tag => {
        current_tag.push(c);
      }
      _ if !in_tag && !in_link => {
        result.push(c);
      }
      _ => {}
    }
  }

  // Clean up whitespace
  result
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn extract_body(parsed: &mailparse::ParsedMail) -> String {
  extract_body_recursive(parsed)
}

fn extract_body_recursive(parsed: &mailparse::ParsedMail) -> String {
  // Try to get body from this part first
  if parsed.ctype.mimetype.starts_with("text/plain") {
    if let Ok(body) = parsed.get_body() {
      if !body.trim().is_empty() {
        return body;
      }
    }
  }

  // If this part has subparts, search them recursively
  if !parsed.subparts.is_empty() {
    // First try to find text/plain
    for subpart in &parsed.subparts {
      if subpart.ctype.mimetype.starts_with("text/plain") {
        if let Ok(body) = subpart.get_body() {
          if !body.trim().is_empty() {
            return body;
          }
        }
      }
    }

    // If no text/plain found, try text/html and convert to plain text
    for subpart in &parsed.subparts {
      if subpart.ctype.mimetype.starts_with("text/html") {
        if let Ok(body) = subpart.get_body() {
          if !body.trim().is_empty() {
            // Clean HTML and extract text
            let cleaned = ammonia::clean(&body);
            let text = html_to_text(&cleaned);
            return text;
          }
        }
      }
    }

    // Recursively search subparts
    for subpart in &parsed.subparts {
      let body = extract_body_recursive(subpart);
      if !body.trim().is_empty() {
        return body;
      }
    }
  }

  // Fallback: try to get any body
  if let Ok(body) = parsed.get_body() {
    return body;
  }

  String::new()
}
