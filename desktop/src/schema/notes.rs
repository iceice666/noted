use polodb_core::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub channel_id: Uuid,
    pub content_id: Uuid,
}
