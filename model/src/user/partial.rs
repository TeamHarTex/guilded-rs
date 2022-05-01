//! The partial user object.

use serde::{Deserialize, Serialize};

use super::UserType;
use crate::id::{marker::UserMarker, Id};

/// Represents a partial user object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    avatar: Option<String>,
    id: Id<UserMarker>,
    name: String,
    r#type: UserType,
}

impl PartialUser {
    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    pub fn id(&self) -> Id<UserMarker> {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn r#type(&self) -> UserType {
        self.r#type.clone()
    }
}
