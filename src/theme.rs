use iced::widget::button;
use iced::Color;

pub const SELECTED_BACKGROUND: Color = Color::from_rgb(0.2, 0.3, 0.4);
pub const MESSAGE_LIST_BACKGROUND: Color = Color::from_rgb(0.15, 0.15, 0.15);
pub const BUTTON_BACKGROUND: Color = Color::from_rgb(0.25, 0.25, 0.25);

pub fn button_style(_theme: &iced::Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(BUTTON_BACKGROUND.into()),
        ..Default::default()
    }
}
