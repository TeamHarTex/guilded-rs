//! The user object.

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

#[derive(Clone, Debug)]
pub struct User {
    id: Id<UserMarker>,
    r#type: UserType,
    name: String,
    created_at: Timestamp,
}

impl User {
    pub fn id(&self) -> Id<UserMarker> {
        self.id.clone()
    }

    pub fn r#type(&self) -> UserType {
        self.r#type.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }
}

/// Represents the type of a user.
#[derive(Clone, Debug)]
pub enum UserType {
    Bot,
    User,
}
