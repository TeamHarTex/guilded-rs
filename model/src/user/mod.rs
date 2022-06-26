//! The user object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

pub mod partial;

/// Represents a user.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub created_at: Timestamp,
    pub id: Id<UserMarker>,
    pub name: String,
    pub r#type: UserType,
}

/// Represents the type of a user.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Bot,
    User,
}
