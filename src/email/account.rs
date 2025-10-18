use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
  pub name: String,
  pub email: String,
  pub imap_server: String,
  pub imap_port: u16,
  pub username: String,
  pub password: String,
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
  pub from: String,
  pub subject: String,
  pub preview: String,
  pub body: String,
  #[allow(dead_code)]
  pub unread: bool,
}
