use noted_desktop::{gui, setup_logger};

fn main() -> iced::Result {
    // Setup
    setup_logger();

    // Launch the GUI
    iced::application("Noted Desktop", gui::update, gui::view).run_with(gui::AppState::new)?;

    // Cleanup

    Ok(())
}
