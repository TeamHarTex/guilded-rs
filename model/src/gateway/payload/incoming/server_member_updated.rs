//! The TeamMemberUpdated event.

use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ServerMarker, UserMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberUpdated {
    server_id: Id<ServerMarker>,
    user_info: UpdatedUserInfo,
}

impl ServerMemberUpdated {
    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn user_info(&self) -> UpdatedUserInfo {
        self.user_info.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedUserInfo {
    id: Id<UserMarker>,
    nickname: String,
}

impl UpdatedUserInfo {
    pub fn id(&self) -> Id<UserMarker> {
        self.id.clone()
    }

    pub fn nickname(&self) -> &str {
        self.nickname.as_ref()
    }
}
