//! The member object.

use crate::datetime::Timestamp;
use crate::id::{marker::RoleMarker, Id};
use crate::user::User;

/// Represents a server member.
#[derive(Clone, Debug)]
pub struct Member {
    user: User,
    role_ids: Vec<Id<RoleMarker>>,
    nickname: Option<String>,
    joined_at: Timestamp,
}

impl Member {
    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn role_ids(&self) -> Vec<Id<RoleMarker>> {
        self.role_ids.clone()
    }

    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }

    pub fn joined_at(&self) -> Timestamp {
        self.joined_at
    }
}
