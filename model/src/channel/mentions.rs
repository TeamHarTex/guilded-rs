//! The Mention object.

use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ChannelMarker, RoleMarker, UserMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mentions {
    pub channels: Option<Vec<ChannelMention>>,
    pub everyone: Option<bool>,
    pub here: Option<bool>,
    pub roles: Option<Vec<RoleMention>>,
    pub users: Option<Vec<UserMention>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelMention {
    id: Id<ChannelMarker>,
}

impl Deref for ChannelMention {
    type Target = Id<ChannelMarker>;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleMention {
    id: Id<RoleMarker>,
}

impl Deref for RoleMention {
    type Target = Id<RoleMarker>;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserMention {
    id: Id<UserMarker>,
}

impl Deref for UserMention {
    type Target = Id<UserMarker>;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}