use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{marker::UserMarker, Id};

/// Represents partial notes of a partial list item.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialListItemNote {
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub updated_at: Option<Timestamp>,
    pub updated_by: Option<Id<UserMarker>>,
}
