mod channels;
mod content;
mod root;

pub use noted_desktop_macros::Preview;
pub use root::RootView;

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

pub trait Component {
    type Message;

    fn update(&mut self, message: Self::Message) -> iced::Task<Self::Message>;
    fn view(&self) -> iced::Element<Self::Message>;
    fn preview() -> (Self, iced::Task<Self::Message>)
    where
        Self: Sized,
    {
        unimplemented!(
            "You have to implement `Component::preview` for your component so that it can be previewed."
        );
    }
}
