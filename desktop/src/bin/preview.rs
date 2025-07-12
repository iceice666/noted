use inquire::{InquireError, Select};
use noted_desktop::{gui::PREVIEW_TARGETS, setup_logger};

fn execute_target(target_name: &str) -> anyhow::Result<()> {
    let (_, func) = PREVIEW_TARGETS
        .iter()
        .find(|(name, _)| *name == target_name)
        .ok_or_else(|| anyhow::anyhow!("Target '{}' not found.", target_name))?;

    func()?;
    Ok(())
}

fn get_target_name() -> anyhow::Result<String> {
    if let Some(target) = std::env::args().nth(1) {
        return Ok(target);
    }

    let options: Vec<&str> = PREVIEW_TARGETS.iter().map(|(name, _)| *name).collect();

    Select::new("Select a target to execute:", options)
        .prompt()
        .map(|s| s.to_string())
        .map_err(|e| match e {
            InquireError::OperationCanceled => {
                println!("Operation canceled.");
                anyhow::anyhow!("Operation canceled")
            }
            _ => anyhow::anyhow!(e),
        })
}

fn main() -> anyhow::Result<()> {
    setup_logger();

    if PREVIEW_TARGETS.is_empty() {
        println!("No targets available.");
        return Ok(());
    }

    let target_name = get_target_name()?;
    execute_target(&target_name)
}
