//! The channel object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{CategoryMarker, ChannelMarker, GroupMarker, ServerMarker, UserMarker},
    Id,
};

pub mod mentions;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerChannel {
    pub archived_at: Option<Timestamp>,
    pub archived_by: Option<Id<UserMarker>>,
    pub category_id: Option<Id<CategoryMarker>>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub group_id: Id<GroupMarker>,
    pub id: Id<ChannelMarker>,
    pub is_public: Option<bool>,
    pub name: String,
    pub parent_id: Option<Id<ChannelMarker>>,
    pub server_id: Id<ServerMarker>,
    pub topic: Option<String>,
    pub r#type: ServerChannelType,
    pub updated_at: Option<Timestamp>,
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
