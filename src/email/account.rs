use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
  pub name: String,
  pub email: String,
  pub imap_server: String,
  pub imap_port: u16,
  #[serde(default = "default_auth")]
  pub auth: String,
  #[serde(default)]
  pub username: String,
  #[serde(default)]
  pub password: String,
  #[serde(default)]
  pub client_id: String,
  #[serde(default)]
  pub client_secret: String,
  #[serde(default)]
  pub refresh_token: String,
}

fn default_auth() -> String {
  "password".to_string()
}

impl AccountConfig {
  pub fn is_oauth(&self) -> bool {
    self.auth == "google_oauth"
  }
}

#[derive(Debug, Clone)]
pub struct Account {
  pub name: String,
  pub email: String,
  pub unread: u32,
  pub folders: Vec<String>,
  pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
  pub uid: u32,
  pub from: String,
  pub subject: String,
  pub preview: String,
  pub body: Option<String>,
  pub date: DateTime<Utc>,
  pub unread: bool,
}
