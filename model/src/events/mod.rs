//! The calendar event object.

use serde::{Deserialize, Serialize};

use crate::channel::mentions::Mentions;
use crate::datetime::Timestamp;
use crate::id::{
    marker::{CalendarEventMarker, ChannelMarker, ServerMarker, UserMarker},
    Id,
};

/// Represents a calendar event.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    cancellation: Option<CalendarEventCancellation>,
    channel_id: Id<ChannelMarker>,
    color: Option<u32>,
    created_at: Timestamp,
    created_by: Id<UserMarker>,
    description: Option<String>,
    duration: Option<u32>,
    id: Id<CalendarEventMarker>,
    is_private: Option<bool>,
    location: Option<String>,
    mentions: Option<Mentions>,
    name: String,
    server_id: Id<ServerMarker>,
    starts_at: Timestamp,
    url: Option<String>,
}

impl CalendarEvent {
    pub fn cancellation(&self) -> Option<CalendarEventCancellation> {
        self.cancellation.clone()
    }

    pub fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id.clone()
    }

    pub fn color(&self) -> Option<u32> {
        self.color
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn created_by(&self) -> Id<UserMarker> {
        self.created_by.clone()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn duration(&self) -> Option<u32> {
        self.duration
    }

    pub fn id(&self) -> Id<CalendarEventMarker> {
        self.id.clone()
    }

    pub fn is_private(&self) -> Option<bool> {
        self.is_private
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn mentions(&self) -> Option<Mentions> {
        self.mentions.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }

    pub fn starts_at(&self) -> Timestamp {
        self.starts_at
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

/// Represents a calendar event cancellation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventCancellation {
    created_by: Option<Id<UserMarker>>,
    description: Option<String>,
}

impl CalendarEventCancellation {
    pub fn created_by(&self) -> Option<Id<UserMarker>> {
        self.created_by.clone()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
