//! The partial user object.

use serde::{Deserialize, Serialize};

use super::UserType;
use crate::id::{marker::UserMarker, Id};

/// Represents a partial user object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    pub avatar: Option<String>,
    pub id: Id<UserMarker>,
    pub name: String,
    pub r#type: UserType,
}
