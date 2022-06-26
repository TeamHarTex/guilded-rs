//! The list item object.

use serde::{Deserialize, Serialize};

use crate::channel::mentions::Mentions;
use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, ListItemMarker, ServerMarker, UserMarker, WebhookMarker},
    Id,
};

pub mod note;
pub mod partial;

/// Represents a list item.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub channel_id: Id<ChannelMarker>,
    pub completed_at: Option<Timestamp>,
    pub completed_by: Option<Id<UserMarker>>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub created_by_webhook_id: Option<Id<WebhookMarker>>,
    pub id: Id<ListItemMarker>,
    pub mentions: Option<Mentions>,
    pub message: String,
    pub note: Option<note::ListItemNote>,
    pub parent_list_item_id: Id<ListItemMarker>,
    pub server_id: Id<ServerMarker>,
    pub updated_at: Option<Timestamp>,
    pub updated_by: Option<Id<UserMarker>>,
}
