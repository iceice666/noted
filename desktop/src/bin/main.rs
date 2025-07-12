use iced::Task;
use noted_desktop::{Component, config, gui::RootView, schema::Database, setup_logger};

fn main() -> iced::Result {
    // Setup
    setup_logger();

    // Launch the GUI
    let config = config::load_config();
    let database = Database::new(&config);
    let app_view = RootView::new(config, database);
    iced::application("Noted Desktop", RootView::update, RootView::view)
        .run_with(|| (app_view, Task::none()))?;

    // Cleanup

    Ok(())
}
