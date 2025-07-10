use iced::widget::{column, text};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {}
}

pub fn view(state: &State) -> Element<Message> {
    column![
        text("Content Area").size(24),
        text("This is where the content will be displayed."),
    ]
    .into()
}
