use crate::email::account::Account;
use crate::theme;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

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
    let toolbar = row![button(text("+"))
        .style(theme::button_style)
        .on_press(Message::AddAccount)]
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
                background: Some(theme::SELECTED_BACKGROUND.into()),
                text_color: theme::TEXT_COLOR,
                ..Default::default()
            });
        } else {
            btn = btn.style(theme::button_style);
        }

        btn = btn.on_press(Message::InboxClicked(index));

        sidebar_content = sidebar_content.push(btn);
    }

    // Section 2: Account details with folders
    for (index, account) in accounts.iter().enumerate() {
        let email_btn = button(text(&account.email))
            .width(Length::Fill)
            .style(theme::button_style)
            .on_press(Message::ToggleAccountExpansion(index));
        sidebar_content = sidebar_content.push(email_btn);

        if *expanded_accounts.get(index).unwrap_or(&false) {
            for folder in &account.folders {
                let folder_btn = button(text(folder))
                    .width(Length::Fill)
                    .style(theme::button_style);
                sidebar_content = sidebar_content.push(folder_btn);
            }
        }
    }

    container(scrollable(sidebar_content))
        .width(250)
        .height(Length::Fill)
        .into()
}
