//! The TeamMemberUnbanned event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::server::member::ban::ServerMemberBan;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberUnbanned {
    pub server_member_ban: ServerMemberBan,
    pub server_id: Id<ServerMarker>,
}
