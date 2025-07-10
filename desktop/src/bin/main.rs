use noted_desktop::{
    gui::{self},
    setup_logger,
};

fn main() -> iced::Result {
    // Setup
    setup_logger();

    // Launch the GUI
    iced::application("Noted Desktop", gui::update, gui::view).run()?;

    // Cleanup

    Ok(())
}
