//! The TeamMemberUpdated event.

use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ServerMarker, UserMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMemberUpdated {
    pub server_id: Id<ServerMarker>,
    pub user_info: UpdatedUserInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedUserInfo {
    pub id: Id<UserMarker>,
    pub nickname: String,
}
