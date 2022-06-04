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
    about: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    created_at: Timestamp,
    default_channel_id: Option<Id<ChannelMarker>>,
    id: Id<ServerMarker>,
    is_verified: Option<bool>,
    name: String,
    owner_id: Id<UserMarker>,
    timezone: Option<String>,
    r#type: ServerType,
    url: Option<String>,
}

impl Server {
    pub fn about(&self) -> Option<&str> {
        self.about.as_deref()
    }

    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    pub fn banner(&self) -> Option<&str> {
        self.banner.as_deref()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn default_channel_id(&self) -> Option<Id<ChannelMarker>> {
        self.default_channel_id.clone()
    }

    pub fn id(&self) -> Id<ServerMarker> {
        self.id.clone()
    }

    pub fn is_verified(&self) -> Option<bool> {
        self.is_verified
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id.clone()
    }

    pub fn timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    pub fn r#type(&self) -> ServerType {
        self.r#type
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
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
