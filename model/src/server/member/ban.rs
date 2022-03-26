//! The server member ban object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};
use crate::user::partial::PartialUser;

/// Represents a server member ban object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberBan {
    user: PartialUser,
    reason: Option<String>,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
}

impl MemberBan {
    pub fn user(&self) -> PartialUser {
        self.user.clone()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }
}
