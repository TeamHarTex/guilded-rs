//! The channel object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{CategoryMarker, ChannelMarker, GroupMarker, ServerMarker, UserMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerChannel {
    archived_at: Option<Timestamp>,
    archived_by: Option<Id<UserMarker>>,
    category_id: Option<Id<CategoryMarker>>,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    group_id: Id<GroupMarker>,
    id: Id<ChannelMarker>,
    is_public: Option<bool>,
    name: String,
    parent_id: Option<Id<ChannelMarker>>,
    server_id: Id<ServerMarker>,
    topic: Option<String>,
    r#type: ServerChannelType,
    updated_at: Option<Timestamp>,
}

impl ServerChannel {
    pub fn archived_at(&self) -> Option<Timestamp> {
        self.archived_at
    }

    pub fn archived_by(&self) -> Option<Id<UserMarker>> {
        self.archived_by.clone()
    }

    pub fn category_id(&self) -> Option<Id<CategoryMarker>> {
        self.category_id.clone()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn group_id(&self) -> Id<GroupMarker> {
        self.group_id.clone()
    }

    pub fn id(&self) -> Id<ChannelMarker> {
        self.id.clone()
    }

    pub fn is_public(&self) -> Option<bool> {
        self.is_public
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn parent_id(&self) -> Option<Id<ChannelMarker>> {
        self.parent_id.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn topic(&self) -> Option<&str> {
        self.topic.as_deref()
    }

    pub fn r#type(&self) -> ServerChannelType {
        self.r#type.clone()
    }

    pub fn updated_at(&self) -> Option<Timestamp> {
        self.updated_at
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerChannelType {
    Announcements,
    Calendar,
    Chat,
    Docs,
    Forums,
    List,
    Media,
    Scheduling,
    Stream,
    Voice,
}
