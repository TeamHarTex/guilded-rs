use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{EmoteMarker, ForumTopicMarker, MessageMarker, UserMarker},
    Id,
};

/// Represents a forum topic reaction object.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicReaction {
    pub channel_id: Id<ForumTopicMarker>,
    pub created_by: Id<UserMarker>,
    pub emote_id: Id<EmoteMarker>,
    pub forum_topic_id: Id<ForumTopicMarker>,
}
