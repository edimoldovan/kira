use crate::email::account::Message as EmailMessage;
use crate::theme;
use chrono::{DateTime, Datelike, Local, Utc};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Border, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
  MessageClicked(usize),
  AddMessage,
}

pub fn view<'a>(messages: &'a [EmailMessage], current_message: usize) -> Element<'a, Message> {
  let toolbar = row![button(text("+"))
    .style(theme::button_style)
    .on_press(Message::AddMessage)]
  .padding(0)
  .spacing(0);

  let mut list_content = column![toolbar].spacing(8).padding(12);

  for (index, msg) in messages.iter().enumerate() {
    let is_selected = index == current_message;

    let date_str = format_date(&msg.date);

    let msg_view = column![
      row![
        text(&msg.from).size(14).width(Length::Fill),
        text(date_str).size(12).color(theme::DIM_COLOR),
      ]
      .width(Length::Fill),
      text(&msg.subject).size(14),
      text(&msg.preview).size(12).color(theme::DIM_COLOR),
    ]
    .spacing(4)
    .padding(8);

    let mut btn = button(msg_view).width(Length::Fill);

    if is_selected {
      btn = btn.style(|_theme, _status| button::Style {
        background: Some(theme::SELECTED_BACKGROUND.into()),
        text_color: theme::TEXT_COLOR,
        ..Default::default()
      });
    } else {
      btn = btn.style(theme::button_style);
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

fn format_date(date: &DateTime<Utc>) -> String {
  let local: DateTime<Local> = date.with_timezone(&Local);
  let now = Local::now();

  let duration = now.signed_duration_since(local);

  if duration.num_days() == 0 {
    local.format("%H:%M").to_string()
  } else if duration.num_days() == 1 {
    "Yesterday".to_string()
  } else if duration.num_days() < 7 {
    local.format("%A").to_string()
  } else if local.year() == now.year() {
    local.format("%b %d").to_string()
  } else {
    local.format("%b %d, %Y").to_string()
  }
}
