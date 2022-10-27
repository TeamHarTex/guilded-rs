//! The ForumTopicReactionCreated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::reaction::forum_topic::ForumTopicReaction;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicReactionCreated {
    pub reaction: ForumTopicReaction,
    pub server_id: Id<ServerMarker>,
}
