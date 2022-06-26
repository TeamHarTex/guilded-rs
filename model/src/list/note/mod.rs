//! The list item note.

use serde::{Deserialize, Serialize};

use crate::channel::mentions::Mentions;
use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

pub mod partial;

/// Represents notes of a list item.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemNote {
    pub content: String,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub mentions: Option<Mentions>,
    pub updated_at: Option<Timestamp>,
    pub updated_by: Option<Id<UserMarker>>,
}
