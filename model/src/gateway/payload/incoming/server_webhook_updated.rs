//! The TeamWebhookUpdated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::webhook::Webhook;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerWebhookUpdated {
    server_id: Id<ServerMarker>,
    webhook: Webhook,
}

impl ServerWebhookUpdated {
    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn webhook(&self) -> Webhook {
        self.webhook.clone()
    }
}

