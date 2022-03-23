//! The user object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

/// Represents a user.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Bot,
    User,
}
