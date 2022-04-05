//! The TeamMemberRemoved event.

use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ServerMarker, UserMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberRemoved {
    is_ban: bool,
    is_kick: bool,
    server_id: Id<ServerMarker>,
    user_id: Id<UserMarker>,
}
