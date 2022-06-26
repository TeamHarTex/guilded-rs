//! The webhook object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ChannelMarker, ServerMarker, UserMarker, WebhookMarker},
    Id,
};

/// Represents a webhook object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub channel_id: Id<ChannelMarker>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub deleted_at: Option<Timestamp>,
    pub id: Id<WebhookMarker>,
    pub name: String,
    pub server_id: Id<ServerMarker>,
    pub token: Option<String>,
}
