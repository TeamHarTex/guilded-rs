//! The emote object.

use serde::{Deserialize, Serialize};

use crate::id::{marker::EmoteMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    pub id: Id<EmoteMarker>,
    pub name: String,
    pub url: String,
}
