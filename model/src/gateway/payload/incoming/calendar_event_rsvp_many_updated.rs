//! The CalendarEventRsvpManyUpdated event.

use serde::{Deserialize, Serialize};

use crate::event::rsvp::CalendarEventRsvp;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventRsvpManyUpdated {
    pub calendar_event_rsvps: Vec<CalendarEventRsvp>,
    pub server_id: Id<ServerMarker>,
}
