//! The partial member object.

use serde::{Deserialize, Serialize};

use crate::id::{marker::RoleMarker, Id};
use crate::user::partial::PartialUser;

/// Represents a partial member object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialMember {
    pub role_ids: Vec<Id<RoleMarker>>,
    pub user: PartialUser,
}
