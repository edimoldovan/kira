mod config;
mod email;
mod theme;
mod ui;

use email::account::Account;
use email::imap;
use iced::widget::{container, row};
use iced::{Element, Length, Task, Theme};

fn main() -> iced::Result {
  iced::application("Kira", Kira::update, Kira::view)
    .theme(Kira::theme)
    .window_size((1200.0, 800.0))
    .run_with(Kira::new)
}

struct Kira {
  accounts: Vec<Account>,
  current_account: usize,
  current_message: usize,
  expanded_accounts: Vec<bool>,
  sync_error: Option<String>,
  is_syncing: bool,
}

#[derive(Debug, Clone)]
enum Message {
  Sidebar(ui::sidebar::Message),
  MessageList(ui::message_list::Message),
  Reader(ui::reader::Message),
  SyncComplete(Result<(), String>),
}

impl Kira {
  fn new() -> (Self, Task<Message>) {
    let accounts = config::load_accounts();
    let expanded_accounts = vec![false; accounts.len()];

    let sync_task = Task::perform(sync_all_accounts(), Message::SyncComplete);

    (
      Self {
        accounts,
        current_account: 0,
        current_message: 0,
        expanded_accounts,
        sync_error: None,
        is_syncing: true,
      },
      sync_task,
    )
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Sidebar(sidebar_msg) => match sidebar_msg {
        ui::sidebar::Message::InboxClicked(index) => {
          self.current_account = index;
          self.current_message = 0;
        }
        ui::sidebar::Message::AddAccount => {}
        ui::sidebar::Message::ToggleAccountExpansion(index) => {
          if let Some(expanded) = self.expanded_accounts.get_mut(index) {
            *expanded = !*expanded;
          }
        }
      },
      Message::MessageList(msg_list_msg) => match msg_list_msg {
        ui::message_list::Message::MessageClicked(index) => {
          self.current_message = index;
        }
        ui::message_list::Message::AddMessage => {}
      },
      Message::Reader(_reader_msg) => {}
      Message::SyncComplete(result) => {
        self.is_syncing = false;
        match result {
          Ok(_) => {
            self.accounts = config::load_accounts();
            self.expanded_accounts = vec![false; self.accounts.len()];
            self.sync_error = None;
          }
          Err(e) => {
            self.sync_error = Some(e);
          }
        }
      }
    }
    Task::none()
  }

  fn view(&self) -> Element<'_, Message> {
    let sidebar = ui::sidebar::view(
      &self.accounts,
      self.current_account,
      &self.expanded_accounts,
      self.sync_error.as_deref(),
      self.is_syncing,
    )
    .map(Message::Sidebar);

    let messages = self
      .accounts
      .get(self.current_account)
      .map(|acc| acc.messages.as_slice())
      .unwrap_or(&[]);

    let message_list =
      ui::message_list::view(messages, self.current_message).map(Message::MessageList);

    let current_msg = self
      .accounts
      .get(self.current_account)
      .and_then(|acc| acc.messages.get(self.current_message));

    let reader = ui::reader::view(current_msg).map(Message::Reader);

    let content = row![sidebar, message_list, reader]
      .spacing(0)
      .width(Length::Fill)
      .height(Length::Fill);

    container(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
  }

  fn theme(&self) -> Theme {
    Theme::Dark
  }
}

async fn sync_all_accounts() -> Result<(), String> {
  let configs = config::get_account_configs();
  let mut errors = Vec::new();

  for account in &configs {
    if let Err(e) = imap::sync_account(account).await {
      let error_msg = format!("{}: {}", account.email, e);
      eprintln!("Failed to sync {}", error_msg);
      errors.push(error_msg);
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors.join("\n"))
  }
}
