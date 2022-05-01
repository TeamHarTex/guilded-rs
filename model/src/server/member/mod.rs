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
    joined_at: Timestamp,
    nickname: Option<String>,
    role_ids: Vec<Id<RoleMarker>>,
    user: User,
}

impl ServerMember {
    pub fn joined_at(&self) -> Timestamp {
        self.joined_at
    }

    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }

    pub fn role_ids(&self) -> Vec<Id<RoleMarker>> {
        self.role_ids.clone()
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }
}
