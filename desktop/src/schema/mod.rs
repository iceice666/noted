mod channels;
mod notes;

use crate::config::AppConfig;
use polodb_core::{Collection, Database as PoloDatabase};
use std::path::PathBuf;

pub use channels::*;
pub use notes::*;

pub struct Database {
    db: PoloDatabase,
    // collections
    channels_collection: Collection<channels::Channels>,
}

impl Database {
    pub fn new(config: &AppConfig) -> Self {
        let db_path = Self::get_db_path(config);

        tracing::debug!("Opening PoloDB database at {}", db_path.display());

        let db = match PoloDatabase::open_file(db_path) {
            Ok(db) => {
                tracing::debug!("Successfully opened PoloDB database");
                db
            }
            Err(e) => {
                panic!("Failed to open PoloDB database: {e}");
            }
        };

        // Get collections
        let channels_collection = db.collection(channels::COLLECTION_NAME);

        Self {
            db,
            channels_collection,
        }
    }

    fn get_db_path(config: &AppConfig) -> PathBuf {
        let path = if let Some(custom_path) = &config.storage.path {
            PathBuf::from(custom_path).join("noted.db")
        } else {
            dirs::data_dir()
                .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
                .join("noted")
                .join("noted.db")
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                panic!(
                    "Failed to create database directory {}: {}",
                    parent.display(),
                    e
                );
            }
        }

        path
    }
}
