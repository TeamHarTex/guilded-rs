//! The user object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

pub mod partial;

/// Represents a user.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    avatar: Option<String>,
    banner: Option<String>,
    created_at: Timestamp,
    id: Id<UserMarker>,
    name: String,
    r#type: UserType,
}

impl User {
    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    pub fn banner(&self) -> Option<&str> {
        self.banner.as_deref()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn id(&self) -> Id<UserMarker> {
        self.id.clone()
    }

    pub fn r#type(&self) -> UserType {
        self.r#type
    }
}

/// Represents the type of a user.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Bot,
    User,
}
