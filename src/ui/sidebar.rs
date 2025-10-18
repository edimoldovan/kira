use crate::email::account::Account;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Color, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    InboxClicked(usize),
    AddAccount,
    ToggleAccountExpansion(usize),
}

pub fn view<'a>(
    accounts: &'a [Account],
    current_account: usize,
    expanded_accounts: &'a [bool],
) -> Element<'a, Message> {
    let toolbar = row![button(text("+")).on_press(Message::AddAccount)]
        .padding(0)
        .spacing(0);

    let mut sidebar_content = column![toolbar].spacing(8).padding(12);

    // Section 1: Inbox buttons
    for (index, account) in accounts.iter().enumerate() {
        let is_selected = index == current_account;
        let mut btn = button(text(format!("{} ({})", account.name, account.unread)))
            .width(Length::Fill);

        if is_selected {
            btn = btn.style(|_theme, _status| button::Style {
                background: Some(Color::from_rgb(0.3, 0.5, 0.8).into()),
                ..Default::default()
            });
        }

        btn = btn.on_press(Message::InboxClicked(index));

        sidebar_content = sidebar_content.push(btn);
    }

    // Section 2: Account details with folders
    for (index, account) in accounts.iter().enumerate() {
        let email_btn = button(text(&account.email))
            .width(Length::Fill)
            .on_press(Message::ToggleAccountExpansion(index));
        sidebar_content = sidebar_content.push(email_btn);

        if *expanded_accounts.get(index).unwrap_or(&false) {
            for folder in &account.folders {
                let folder_btn = button(text(folder)).width(Length::Fill);
                sidebar_content = sidebar_content.push(folder_btn);
            }
        }
    }

    container(scrollable(sidebar_content))
        .width(250)
        .height(Length::Fill)
        .into()
}
