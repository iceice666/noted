use noted_desktop::{gui::PREVIEW_TARGETS, setup_logger};

fn main() -> iced::Result {
    setup_logger();

    let target = std::env::args()
        .nth(1)
        .expect("No preview target specified");

    let f = PREVIEW_TARGETS
        .iter()
        .find(|(name, _)| name == &target)
        .map(|(_, func)| func)
        .expect("Preview target not found");

    f()
}
