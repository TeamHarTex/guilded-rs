//! The Guild object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, ServerMarker, UserMarker},
    Id,
};

pub mod member;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub about: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub created_at: Timestamp,
    pub default_channel_id: Option<Id<ChannelMarker>>,
    pub id: Id<ServerMarker>,
    pub is_verified: Option<bool>,
    pub name: String,
    pub owner_id: Id<UserMarker>,
    pub timezone: Option<String>,
    pub r#type: ServerType,
    pub url: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    Clan,
    Community,
    Friends,
    Guild,
    Organization,
    Other,
    Streaming,
    Team,
}
