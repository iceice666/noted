// Template for the new component
/*
use iced::{Element, Task};

use crate::gui::AppState;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {}
}

pub fn view(state: &AppState) -> Element<Message> {
    unimplemented!()
}
*/

use iced::{
    Element, Task,
    widget::{row, vertical_rule},
    window,
};

use crate::{
    gui::{AppState, channels, content},
    preview,
};

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    Channels(super::channels::Message),
    Content(super::content::Message),
}

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
        Message::Channels(message) => channels::update(state, message).map(Message::Channels),
    }
}

pub fn view(state: &AppState) -> Element<Message> {
    row![
        channels::view(state).map(Message::Channels),
        vertical_rule(0),
        content::view(state).map(Message::Content),
    ]
    .into()
}

preview!();
