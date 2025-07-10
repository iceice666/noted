// Template for the new component
/*
use iced::{Element, Task};

use crate::gui::AppState;

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
    unimplemented!()
}
*/

mod channels;
mod content;
mod root;

pub use root::State as AppState;
pub use root::{update, view};

pub fn run() -> iced::Result {
    iced::application("Noted Desktop", root::update, root::view).run()
}

//
// Preview feature for running specific targets
//
#[cfg(feature = "preview")]
#[linkme::distributed_slice]
pub static PREVIEW_TARGETS: [(&'static str, fn() -> iced::Result)];

#[cfg(feature = "preview")]
pub fn run_preview(target: &str) -> anyhow::Result<()> {
    let result = std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("preview")
        .arg("--features")
        .arg("preview")
        .arg("--")
        .arg(target)
        .status()?;

    if result.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to previewing {}", target))
    }
}

#[macro_export]
macro_rules! preview {
    (state) => {
        preview!(state = AppState);
    };

    () => {
        preview!(state = State);
    };

    (state = $state:ident) => {
        struct __Title;
        impl iced::application::Title<$state> for __Title {
            fn title(&self, _state: &$state) -> String {
                format!("Previewing {}", module_path!())
            }
        }

        #[cfg(feature = "preview")]
        fn __run() -> iced::Result {
            iced::application(__Title, update, view).run()
        }

        #[cfg(feature = "preview")]
        #[linkme::distributed_slice($crate::gui::PREVIEW_TARGETS)]
        static __CONFIG: (&str, fn() -> iced::Result) = (module_path!(), __run);

        #[cfg(feature = "preview")]
        #[test]
        fn preview() -> anyhow::Result<()> {
            $crate::gui::run_preview(module_path!())
        }
    };
}
