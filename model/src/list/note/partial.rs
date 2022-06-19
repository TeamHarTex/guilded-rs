use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

/// Represents partial notes of a partial list item.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialListItemNote {
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    updated_at: Option<Timestamp>,
    updated_by: Option<Id<UserMarker>>,
}

impl PartialListItemNote {
    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn updated_at(&self) -> Option<Timestamp> {
        self.updated_at
    }

    pub fn updated_by(&self) -> Option<Id<UserMarker>> {
        self.updated_by.clone()
    }
}
