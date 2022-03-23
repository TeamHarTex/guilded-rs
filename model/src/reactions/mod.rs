//! The content reaction object.

use serde::{Deserialize, Serialize};

use crate::{
    datetime::Timestamp,
    id::{
        marker::{ReactionMarker, ServerMarker, UserMarker, WebhookMarker},
        Id,
    },
};

/// Represents a reaction object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    created_by_webhook_id: Option<Id<WebhookMarker>>,
    id: Id<ReactionMarker>,
    server_id: Id<ServerMarker>,
}

impl Reaction {
    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn created_by_webhook_id(&self) -> Option<Id<WebhookMarker>> {
        self.created_by_webhook_id.clone()
    }

    pub fn id(&self) -> Id<ReactionMarker> {
        self.id.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }
}
