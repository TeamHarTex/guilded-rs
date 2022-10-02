//! The ForumTopicUnpinned event.

use serde::{Deserialize, Serialize};

use crate::forum::topic::ForumTopic;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicUnpinned {
    pub id: Id<ServerMarker>,
    pub forum_topic: ForumTopic,
}
