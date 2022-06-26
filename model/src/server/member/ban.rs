//! The server member ban object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};
use crate::user::partial::PartialUser;

/// Represents a server member ban object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberBan {
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub reason: Option<String>,
    pub user: PartialUser,
}
