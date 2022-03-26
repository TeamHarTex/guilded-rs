//! The partial user object.

use super::UserType;
use crate::id::{marker::UserMarker, Id};

/// Represents a partial user object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    id: Id<UserMarker>,
    r#type: UserType,
    name: String,
}

impl PartialUser {
    pub fn id(&self) -> Id<UserMarker> {
        self.id.clone()
    }

    pub fn r#type(&self) -> UserType {
        self.r#type.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
