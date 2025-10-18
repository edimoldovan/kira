use crate::email::account::Message as EmailMessage;
use crate::theme;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Border, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    MessageClicked(usize),
    AddMessage,
}

pub fn view<'a>(
    messages: &'a [EmailMessage],
    current_message: usize,
) -> Element<'a, Message> {
    let toolbar = row![button(text("+")).on_press(Message::AddMessage)]
        .padding(0)
        .spacing(0);

    let mut list_content = column![toolbar].spacing(8).padding(12);

    for (index, msg) in messages.iter().enumerate() {
        let is_selected = index == current_message;

        let msg_view = column![
            text(&msg.from).size(14),
            text(&msg.subject).size(14),
            text(&msg.preview).size(12),
        ]
        .spacing(4)
        .padding(8);

        let mut btn = button(msg_view).width(Length::Fill);

        if is_selected {
            btn = btn.style(|_theme, _status| button::Style {
                background: Some(theme::SELECTED_BACKGROUND.into()),
                ..Default::default()
            });
        }

        btn = btn.on_press(Message::MessageClicked(index));

        list_content = list_content.push(btn);
    }

    container(scrollable(list_content))
        .width(400)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(theme::MESSAGE_LIST_BACKGROUND.into()),
            border: Border::default(),
            ..Default::default()
        })
        .into()
}
