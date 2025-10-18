use crate::email::account::Message as EmailMessage;
use crate::theme;
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
    button(text("Reply")).style(theme::button_style),
    button(text("Reply all")).style(theme::button_style),
    button(text("Forward")).style(theme::button_style),
    horizontal_space(),
    button(text("Mark as spam")).style(theme::button_style),
    button(text("Delete")).style(theme::button_style),
  ]
  .spacing(8)
  .padding(0);

  let mut reader_content = column![toolbar].spacing(16).padding(12);

  if let Some(msg) = message {
    reader_content = reader_content
      .push(text(format!("From: {}", msg.from)).color(theme::TEXT_COLOR))
      .push(text(format!("Subject: {}", msg.subject)).color(theme::TEXT_COLOR));

    if let Some(body) = &msg.body {
      eprintln!("=== BODY START (len: {}) ===", body.len());
      eprintln!("{}", body);
      eprintln!("=== BODY END ===");
      reader_content = reader_content.push(text(body).color(theme::TEXT_COLOR));
    } else {
      reader_content =
        reader_content.push(text("Loading message body...").color(theme::DIM_COLOR));
    }
  }

  container(scrollable(reader_content))
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
