use std::path::PathBuf;

use anyhow::Result;
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use std::fs;
use tracing::instrument;

macro_rules! app_config {
    (
        $(
            $field_name:ident: $field_type:ty
            $({
                $(
                    $nested_field_name:ident: $nested_field_type:ty $(= $nested_field_default:expr)?
                ),* $(,)?
            })?
            $(= $field_default:expr)?

        ),*
        $(,)?
    ) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct AppConfig {
            $(
                pub $field_name: $field_type
            ),*
        }

        impl Default for AppConfig {
            fn default() -> Self {
                Self {
                    $(
                        $field_name: app_config!(@or_type_default $($field_default)?)
                    ),*
                }
            }
        }

        $(paste::paste!{$(
            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            pub struct [<$field_type>] {
                $(pub $nested_field_name: $nested_field_type),*
            }

            impl Default for [<$field_type>] {
                fn default() -> Self {
                    Self {
                        $(
                            $nested_field_name: app_config!(@or_type_default $($nested_field_default)?)
                        ),*
                    }
                }
            }
        )?})*
    };

    (@or_type_default $value:expr) => {
        $value
    };
    (@or_type_default) => {
        Default::default()
    };

}

app_config! {
    storage: Storage {
        path: Option<String> = None,
    },
}

#[instrument]
pub fn load_config() -> AppConfig {
    let config_path = get_config_path();

    if config_path.exists() {
        return load_existing_config(&config_path).unwrap_or_else(|e| {
            tracing::error!("Failed to load existing config, using defaults: {e}");
            AppConfig::default()
        });
    }

    match create_default_config(&config_path) {
        Ok(()) => load_existing_config(&config_path).unwrap_or_else(|e| {
            tracing::error!("Failed to load newly created config, using defaults: {e}");
            AppConfig::default()
        }),
        Err(e) => {
            tracing::error!("Failed to create default config, using defaults: {e}");
            AppConfig::default()
        }
    }
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .map(|p| p.join("noted/config.toml"))
        .unwrap_or_else(|| {
            dirs::home_dir()
                .map(|p| p.join(".config/noted/config.toml"))
                .unwrap_or_else(|| PathBuf::from("config.toml"))
        })
}

fn load_existing_config(config_path: &PathBuf) -> Result<AppConfig> {
    tracing::debug!("Loading config from {}", config_path.display());
    Figment::new()
        .join(Toml::file(config_path))
        .join(Serialized::defaults(AppConfig::default()))
        .extract::<AppConfig>()
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to load configuration from {}: {}",
                config_path.display(),
                e
            )
        })
}

fn create_default_config(config_path: &PathBuf) -> Result<()> {
    tracing::debug!("Creating default config at {}", config_path.display());

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            anyhow::anyhow!(
                "Failed to create config directory {}: {}",
                parent.display(),
                e
            )
        })?;
    }

    fs::write(config_path, include_bytes!("../config.example.toml")).map_err(|e| {
        anyhow::anyhow!(
            "Failed to create default config at {}: {}",
            config_path.display(),
            e
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::setup_logger;

    use super::*;

    #[test]
    fn test_load() -> Result<()> {
        setup_logger();

        let config = load_config();
        println!("{config:#?}");
        Ok(())
    }
}
