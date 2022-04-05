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

impl ServerMemberRemoved {
    pub fn is_ban(&self) -> bool {
        self.is_ban
    }

    pub fn is_kick(&self) -> bool {
        self.is_kick
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn user_id(&self) -> Id<UserMarker> {
        self.user_id.clone()
    }
}
