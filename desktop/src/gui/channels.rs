use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use crate::gui::AppState;
use crate::preview;

#[derive(Debug, Clone, Copy)]
pub enum Message {}

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {}
}

pub fn view(state: &AppState) -> Element<Message> {
    column![
        text("This").size(24),
        text("is").size(24),
        text("a").size(24),
        text("sidebar").size(24),
    ]
    
    .into()
}



preview!();