//! The emote object.

use serde::{Deserialize, Serialize};

use crate::id::{marker::EmoteMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    id: Id<EmoteMarker>,
    name: String,
    url: String,
}

impl Emote {
    pub fn id(&self) -> Id<EmoteMarker> {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }
}
