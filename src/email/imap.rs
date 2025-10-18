use crate::email::account::{AccountConfig, Message};
use crate::email::cache::{load_folder_cache, save_folder_cache};
use async_imap::types::Flag;
use async_imap::Session;
use async_native_tls::{TlsConnector, TlsStream};
use async_std::net::TcpStream;
use chrono::Utc;
use futures::StreamExt;
use mailparse::{parse_mail, MailHeaderMap};

type ImapSession = Session<TlsStream<TcpStream>>;

pub async fn sync_account(account: &AccountConfig) -> Result<(), String> {
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
      .fetch(&fetch_range, "RFC822")
      .await
      .map_err(|e| format!("Failed to fetch messages: {}", e))?;

    let mut new_messages = Vec::new();
    let mut max_uid = cache.last_uid;

    while let Some(msg_result) = messages_stream.next().await {
      if let Ok(msg) = msg_result {
        if let Some(body) = msg.body() {
          if let Ok(parsed) = parse_mail(body) {
            let message = Message {
              from: parsed
                .headers
                .get_first_value("From")
                .unwrap_or_default(),
              subject: parsed
                .headers
                .get_first_value("Subject")
                .unwrap_or_default(),
              preview: extract_preview(&parsed),
              body: extract_body(&parsed),
              unread: !msg.flags().any(|f| matches!(f, Flag::Seen)),
            };
            new_messages.push(message);
          }
        }

        if let Some(uid) = msg.uid {
          max_uid = max_uid.max(uid);
        }
      }
    }

    cache.messages.extend(new_messages);
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
  let tcp = TcpStream::connect(&addr)
    .await
    .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

  let tls = TlsConnector::new();
  let tls_stream = tls
    .connect(&account.imap_server, tcp)
    .await
    .map_err(|e| format!("TLS connection failed: {}", e))?;

  let client = async_imap::Client::new(tls_stream);

  let session = client
    .login(&account.username, &account.password)
    .await
    .map_err(|e| format!("Login failed: {}", e.0))?;

  Ok(session)
}

async fn get_message_count(session: &mut ImapSession) -> Result<u32, String> {
  let mailbox = session
    .examine("INBOX")
    .await
    .map_err(|e| format!("Failed to examine INBOX: {}", e))?;

  Ok(mailbox.exists)
}

fn extract_preview(parsed: &mailparse::ParsedMail) -> String {
  let body = extract_body(parsed);
  body.chars().take(100).collect::<String>()
}

fn extract_body(parsed: &mailparse::ParsedMail) -> String {
  if let Ok(body) = parsed.get_body() {
    return body;
  }

  for subpart in &parsed.subparts {
    if subpart.ctype.mimetype.starts_with("text/") {
      if let Ok(body) = subpart.get_body() {
        return body;
      }
    }
  }

  String::new()
}
