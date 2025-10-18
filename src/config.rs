use crate::email::account::{Account, AccountConfig, Message};
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
    .map(|config| Account {
      name: config.name,
      email: config.email,
      unread: 3,
      folders: vec![
        "Inbox".to_string(),
        "Sent".to_string(),
        "Drafts".to_string(),
        "Trash".to_string(),
      ],
      messages: mock_messages(),
    })
    .collect()
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

fn mock_messages() -> Vec<Message> {
  vec![
        Message {
            from: "alice@example.com".to_string(),
            subject: "Project Update".to_string(),
            preview: "Hey, I wanted to give you a quick update on the project...".to_string(),
            body: "Hey, I wanted to give you a quick update on the project. We've made great progress this week and are on track for the deadline.".to_string(),
            unread: true,
        },
        Message {
            from: "bob@company.com".to_string(),
            subject: "Meeting Tomorrow".to_string(),
            preview: "Just a reminder about our meeting scheduled for tomorrow...".to_string(),
            body: "Just a reminder about our meeting scheduled for tomorrow at 2 PM. Please bring the quarterly reports.".to_string(),
            unread: true,
        },
        Message {
            from: "notifications@service.com".to_string(),
            subject: "Your Weekly Summary".to_string(),
            preview: "Here's your weekly activity summary...".to_string(),
            body: "Here's your weekly activity summary. You had 47 tasks completed this week. Great job!".to_string(),
            unread: true,
        },
        Message {
            from: "charlie@example.com".to_string(),
            subject: "Re: Question about API".to_string(),
            preview: "Thanks for reaching out! Here's the documentation...".to_string(),
            body: "Thanks for reaching out! Here's the documentation you requested. Let me know if you have any other questions.".to_string(),
            unread: false,
        },
        Message {
            from: "team@startup.io".to_string(),
            subject: "New Feature Release".to_string(),
            preview: "We're excited to announce our latest feature...".to_string(),
            body: "We're excited to announce our latest feature release! Check out the new dashboard and analytics tools.".to_string(),
            unread: false,
        },
    ]
}
