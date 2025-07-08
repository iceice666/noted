use iced::{Element, Task};
use iced::widget::{column, text};

use crate::gui::AppState;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {}
}

pub fn view(state: &AppState) -> Element<Message> {
    column![
        text("Content Area").size(24),
        text("This is where the content will be displayed."),
    ].into()
}

