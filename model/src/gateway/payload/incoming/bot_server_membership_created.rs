//! The BotServerMembershipCreated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::UserMarker, Id};
use crate::server::Server;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotServerMembershipCreated {
    pub created_by: Id<UserMarker>,
    pub server: Server,
}
