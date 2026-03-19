use crate::email::account::{Account, AccountConfig};
use crate::email::cache;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
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
      auth: "password".to_string(),
      username: "work@example.com".to_string(),
      password: "".to_string(),
      client_id: String::new(),
      client_secret: String::new(),
      refresh_token: String::new(),
    },
    AccountConfig {
      name: "Personal".to_string(),
      email: "personal@example.com".to_string(),
      imap_server: "imap.example.com".to_string(),
      imap_port: 993,
      auth: "password".to_string(),
      username: "personal@example.com".to_string(),
      password: "".to_string(),
      client_id: String::new(),
      client_secret: String::new(),
      refresh_token: String::new(),
    },
  ]
}

pub fn save_refresh_token(email: &str, refresh_token: &str) -> Result<(), String> {
  let path = config_path().ok_or("Could not determine config path")?;
  let contents = fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
  let mut config: Config =
    toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))?;

  for account in &mut config.accounts {
    if account.email == email {
      account.refresh_token = refresh_token.to_string();
    }
  }

  let output = toml::to_string_pretty(&config)
    .map_err(|e| format!("Failed to serialize config: {}", e))?;
  fs::write(&path, output).map_err(|e| format!("Failed to write config: {}", e))?;

  Ok(())
}
