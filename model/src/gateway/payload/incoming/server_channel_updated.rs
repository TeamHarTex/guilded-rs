//! The TeamChannelUpdated event.

use serde::{Deserialize, Serialize};

use crate::channel::ServerChannel;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerChannelUpdated {
    pub channel: ServerChannel,
    pub server_id: Id<ServerMarker>,
}
