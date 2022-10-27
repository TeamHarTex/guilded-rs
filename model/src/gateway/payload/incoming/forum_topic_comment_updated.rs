//! The ForumTopicCommentUpdated event.

use serde::{Deserialize, Serialize};

use crate::forum::topic::comment::ForumTopicComment;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicCommentUpdated {
    pub id: Id<ServerMarker>,
    pub forum_topic_comment: ForumTopicComment,
}
