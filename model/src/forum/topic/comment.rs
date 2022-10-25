//! The forum topic comment object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{ForumTopicCommentMarker, ForumTopicMarker, UserMarker},
    Id,
};

/// Represents a forum topic comment.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicComment {
    pub content: String,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub forum_topic_id: Id<ForumTopicMarker>,
    pub id: Id<ForumTopicCommentMarker>,
    pub updated_at: Option<Timestamp>,
}
