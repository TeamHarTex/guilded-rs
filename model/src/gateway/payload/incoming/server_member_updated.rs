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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedUserInfo {
    id: Id<UserMarker>,
    nickname: String,
}
