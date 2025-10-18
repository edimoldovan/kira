use crate::email::account::Message;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCache {
  pub last_sync: Option<DateTime<Utc>>,
  pub last_uid: u32,
  pub messages: Vec<Message>,
}

impl FolderCache {
  pub fn new() -> Self {
    Self {
      last_sync: None,
      last_uid: 0,
      messages: Vec::new(),
    }
  }
}

fn cache_dir() -> Option<PathBuf> {
  ProjectDirs::from("", "", "kira").map(|dirs| {
    let cache_dir = dirs.cache_dir();
    fs::create_dir_all(cache_dir).ok();
    cache_dir.to_path_buf()
  })
}

fn cache_path(account_email: &str, folder: &str) -> Option<PathBuf> {
  cache_dir().map(|dir| {
    let filename = format!("{}_{}.json", account_email.replace('@', "_"), folder);
    dir.join(filename)
  })
}

pub fn load_folder_cache(account_email: &str, folder: &str) -> FolderCache {
  if let Some(path) = cache_path(account_email, folder) {
    if let Ok(contents) = fs::read_to_string(&path) {
      if let Ok(cache) = serde_json::from_str::<FolderCache>(&contents) {
        return cache;
      }
    }
  }
  FolderCache::new()
}

pub fn save_folder_cache(account_email: &str, folder: &str, cache: &FolderCache) -> Result<(), String> {
  let path = cache_path(account_email, folder).ok_or("Failed to get cache path")?;
  let contents = serde_json::to_string_pretty(cache).map_err(|e| e.to_string())?;
  fs::write(&path, contents).map_err(|e| e.to_string())?;
  Ok(())
}

pub fn load_messages(account_email: &str) -> Vec<Message> {
  let cache = load_folder_cache(account_email, "inbox");
  cache.messages.into_iter().take(100).collect()
}
