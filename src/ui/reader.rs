use crate::email::account::Message as EmailMessage;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    Reply,
    ReplyAll,
    Forward,
    Delete,
    MarkAsSpam,
}

pub fn view<'a>(message: Option<&'a EmailMessage>) -> Element<'a, Message> {
    let toolbar = row![
        button(text("Reply")),
        button(text("Reply all")),
        button(text("Forward")),
        horizontal_space(),
        button(text("Mark as spam")),
        button(text("Delete")),
    ]
    .spacing(8)
    .padding(0);

    let mut reader_content = column![toolbar].spacing(16).padding(12);

    if let Some(msg) = message {
        reader_content = reader_content
            .push(text(format!("From: {}", msg.from)))
            .push(text(format!("Subject: {}", msg.subject)))
            .push(text(&msg.body));
    }

    container(scrollable(reader_content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
