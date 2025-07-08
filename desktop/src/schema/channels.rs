use polodb_core::bson::Uuid;
use serde::{Deserialize, Serialize};

pub static COLLECTION_NAME: &str = "channels";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGroup {
    pub name: Option<String>,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channels(Vec<ChannelGroup>);