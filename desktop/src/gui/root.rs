use iced::{
    Element, Task,
    widget::{row, vertical_rule},
    window,
};
use noted_desktop_macros::Preview;

use crate::{
    config::AppConfig,
    gui::{Component, channels, content},
    schema::Database,
};

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    Channels(super::channels::Message),
    Content(super::content::Message),
}


#[Preview]
#[derive(Debug, Default, Clone)]
pub struct RootView {
    pub channels: channels::ChannelsView,
    pub content: content::ContentView,
}

impl RootView {
    pub fn new(_config: AppConfig, _database: Database) -> Self {
        Self {
            channels: channels::ChannelsView::default(),
            content: content::ContentView::default(),
        }
    }
}

impl Component for RootView {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::Channels(msg) => self.channels.update(msg).map(Message::Channels),
            Message::Content(msg) => self.content.update(msg).map(Message::Content),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            self.channels.view().map(Message::Channels),
            vertical_rule(0),
            self.content.view().map(Message::Content),
        ]
        .into()
    }
}
