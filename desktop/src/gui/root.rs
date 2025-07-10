use iced::{
    Element, Task,
    widget::{row, vertical_rule},
    window,
};

use crate::{
    config,
    gui::{channels, content},
    preview,
    schema::Database,
};

pub struct State {
    config: config::AppConfig,
    database: Database,
    // Components state
    channels: channels::State,
    content: content::State,
}

impl Default for State {
    fn default() -> Self {
        let config = config::load_config();
        let database = Database::new(&config);

        Self {
            config,
            database,
            channels: Default::default(),
            content: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    Channels(super::channels::Message),
    Content(super::content::Message),
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
        Message::Channels(message) => {
            channels::update(&mut state.channels, message).map(Message::Channels)
        }
    }
}

pub fn view(state: &State) -> Element<Message> {
    let State {
        channels, content, ..
    } = state;
    row![
        channels::view(channels).map(Message::Channels),
        vertical_rule(0),
        content::view(content).map(Message::Content),
    ]
    .into()
}

preview!();
