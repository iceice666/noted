use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Scrollable, column, text};
use iced::{Element, Task};

use crate::preview;

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
    let lots_of_texts =
        column((0..20).map(|i| text(format!("{} vertical scrollable  ", i + 1)).into()));

    Scrollable::new(lots_of_texts)
        .direction(Direction::Vertical(Scrollbar::new()))
        .into()
}

preview!();
