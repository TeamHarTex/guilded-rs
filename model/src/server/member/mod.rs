//! The server member object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::RoleMarker, Id};
use crate::user::User;

pub mod ban;

/// Represents a server member.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMember {
    pub is_owner: Option<bool>,
    pub joined_at: Timestamp,
    pub nickname: Option<String>,
    pub role_ids: Vec<Id<RoleMarker>>,
    pub user: User,
}
