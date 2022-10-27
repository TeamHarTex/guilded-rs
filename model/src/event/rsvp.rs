//! The calendar event RSVP object.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;
use crate::id::{
    marker::{CalendarEventMarker, ChannelMarker, ServerMarker, UserMarker},
    Id,
};
use crate::user::User;

/// Represents a calendar event RSVP.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventRsvp {
    pub calender_event_id: Id<CalendarEventMarker>,
    pub channel_id: Option<Id<ChannelMarker>>,
    pub created_at: Timestamp,
    pub created_by: Id<UserMarker>,
    pub server_id: Option<Id<ServerMarker>>,
    pub status: RsvpStatus,
    pub updated_at: Option<Timestamp>,
    pub updated_by: Option<Id<User>>,
    pub user_id: Option<Id<UserMarker>>,
}

/// Represents a calendar event RSVP status.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RsvpStatus {
    Declined,
    Going,
    Invited,
    Maybe,
    Waitlisted,
}
