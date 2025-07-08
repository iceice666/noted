use iced::Task;

use crate::{
    config::{self, AppConfig},
    schema::Database,
};

mod channels;
mod content;
mod root;

pub use root::{update, view};

pub struct AppState {
    config: AppConfig,
    database: Database,
}

impl AppState {
    pub fn new<T>() -> (Self, Task<T>) {
        let config = config::load_config();
        let database = Database::new(&config);

        let app = Self { config, database };

        (app, Task::none())
    }
}

pub fn run() -> iced::Result {
    iced::application("Noted Desktop", root::update, root::view).run_with(AppState::new)
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
    () => {
        preview!(state = AppState);
    };

    (state) => {
        preview!(state = State);
    };

    (state = $state:ident) => {
        type __State = $state;

        struct __Title;
        impl iced::application::Title<__State> for __Title {
            fn title(&self, _state: &__State) -> String {
                format!("Previewing {}", module_path!())
            }
        }

        #[cfg(feature = "preview")]
        fn __run() -> iced::Result {
            iced::application(__Title, update, view).run_with(__State::new)
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
