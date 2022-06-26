//! The content reaction object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ReactionMarker, ServerMarker, UserMarker, WebhookMarker},
    Id,
};

/// Represents a reaction object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub created_by_webhook_id: Option<Id<WebhookMarker>>,
    pub id: Id<ReactionMarker>,
    pub server_id: Id<ServerMarker>,
}
