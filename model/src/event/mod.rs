//! The calendar event object.

use serde::{Deserialize, Serialize};

use crate::channel::mentions::Mentions;
use crate::datetime::Timestamp;
use crate::id::{
    marker::{CalendarEventMarker, ChannelMarker, ServerMarker, UserMarker},
    Id,
};

pub mod rsvp;

/// Represents a calendar event.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub cancellation: Option<CalendarEventCancellation>,
    pub channel_id: Id<ChannelMarker>,
    pub color: Option<u32>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub description: Option<String>,
    pub duration: Option<u32>,
    pub id: Id<CalendarEventMarker>,
    pub is_private: Option<bool>,
    pub location: Option<String>,
    pub mentions: Option<Mentions>,
    pub name: String,
    pub rsvp_limit: Option<u64>,
    pub server_id: Id<ServerMarker>,
    pub starts_at: Timestamp,
    pub url: Option<String>,
}

/// Represents a calendar event cancellation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventCancellation {
    pub created_by: Option<Id<UserMarker>>,
    pub description: Option<String>,
}
