//! The TeamMemberJoined event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::server::member::ServerMember;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberJoined {
    member: ServerMember,
    server_id: Id<ServerMarker>,
}
