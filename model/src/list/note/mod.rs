//! The list item note.

use crate::channel::mentions::Mentions;
use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

pub mod partial;

/// Represents notes of a list item.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemNote {
    content: String,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    mentions: Option<Mentions>,
    updated_at: Option<Timestamp>,
    updated_by: Option<Id<UserMarker>>,
}

impl ListItemNote {
    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn mentions(&self) -> Option<Mentions> {
        self.mentions.clone()
    }

    pub fn updated_at(&self) -> Option<Timestamp> {
        self.updated_at
    }

    pub fn updated_by(&self) -> Option<Id<UserMarker>> {
        self.updated_by.clone()
    }
}
