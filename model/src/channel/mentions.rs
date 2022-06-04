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
    channels: Option<Vec<ChannelMention>>,
    everyone: Option<bool>,
    here: Option<bool>,
    roles: Option<Vec<RoleMention>>,
    users: Option<Vec<UserMention>>,
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

impl Mentions {
    pub fn channels(&self) -> Option<Vec<ChannelMention>> {
        self.channels.clone()
    }

    pub fn everyone(&self) -> Option<bool> {
        self.everyone
    }

    pub fn here(&self) -> Option<bool> {
        self.here
    }

    pub fn roles(&self) -> Option<Vec<RoleMention>> {
        self.roles.clone()
    }

    pub fn users(&self) -> Option<Vec<UserMention>> {
        self.users.clone()
    }
}
