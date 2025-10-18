use crate::email::account::{Account, AccountConfig};
use crate::email::cache;
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Config {
  accounts: Vec<AccountConfig>,
}

fn config_path() -> Option<PathBuf> {
  ProjectDirs::from("", "", "kira").map(|dirs| {
    let config_dir = dirs.config_dir();
    fs::create_dir_all(config_dir).ok();
    config_dir.join("accounts.toml")
  })
}

pub fn load_accounts() -> Vec<Account> {
  let configs = load_account_configs();

  configs
    .into_iter()
    .map(|config| {
      let messages = cache::load_messages(&config.email);
      let unread = messages.iter().filter(|m| m.unread).count() as u32;

      Account {
        name: config.name,
        email: config.email,
        unread,
        folders: vec![
          "Inbox".to_string(),
          "Sent".to_string(),
          "Drafts".to_string(),
          "Trash".to_string(),
        ],
        messages,
      }
    })
    .collect()
}

pub fn get_account_configs() -> Vec<AccountConfig> {
  load_account_configs()
}

fn load_account_configs() -> Vec<AccountConfig> {
  if let Some(path) = config_path() {
    match fs::read_to_string(&path) {
      Ok(contents) => match toml::from_str::<Config>(&contents) {
        Ok(config) => return config.accounts,
        Err(_) => {}
      },
      Err(_) => {}
    }
  }

  default_account_configs()
}

fn default_account_configs() -> Vec<AccountConfig> {
  vec![
    AccountConfig {
      name: "Work".to_string(),
      email: "work@example.com".to_string(),
      imap_server: "imap.example.com".to_string(),
      imap_port: 993,
      username: "work@example.com".to_string(),
      password: "".to_string(),
    },
    AccountConfig {
      name: "Personal".to_string(),
      email: "personal@example.com".to_string(),
      imap_server: "imap.example.com".to_string(),
      imap_port: 993,
      username: "personal@example.com".to_string(),
      password: "".to_string(),
    },
  ]
}
