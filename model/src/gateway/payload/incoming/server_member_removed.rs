//! The TeamMemberRemoved event.

use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ServerMarker, UserMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberRemoved {
    pub is_ban: bool,
    pub is_kick: bool,
    pub server_id: Id<ServerMarker>,
    pub user_id: Id<UserMarker>,
}
