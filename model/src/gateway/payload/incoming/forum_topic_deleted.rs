//! The ForumTopicDeleted event.

use serde::{Deserialize, Serialize};

use crate::forum::topic::ForumTopic;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicDeleted {
    pub id: Id<ServerMarker>,
    pub forum_topic: ForumTopic,
}
