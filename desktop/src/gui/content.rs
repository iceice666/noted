use iced::widget::{column, text};
use iced::{Element, Task};

use crate::gui::{Component, Preview};

#[derive(Debug, Clone)]
pub enum Message {}

#[Preview]
#[derive(Debug, Default, Clone)]
pub struct ContentView {}

impl Component for ContentView {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {}
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text("Content Area").size(24),
            text("This is where the content will be displayed."),
        ]
        .into()
    }
}

