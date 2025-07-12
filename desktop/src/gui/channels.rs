use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Scrollable, column, text};
use iced::{Element, Task};

use crate::gui::{Component, Preview};
use crate::schema::Channels;

#[derive(Debug, Clone)]
pub enum Message {}

#[Preview]
#[derive(Debug, Default, Clone)]
pub struct ChannelsView {
    pub channels: Channels,
}

impl Component for ChannelsView {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {}
    }

    fn view(&self) -> Element<Self::Message> {
        let chs = column((0..20).map(|i| text(format!("{} vertical scrollable  ", i + 1)).into()));

        Scrollable::new(chs)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
