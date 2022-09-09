//! The CalendarEventRsvpUpdated event.

use serde::{Deserialize, Serialize};

use crate::event::rsvp::CalendarEventRsvp;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventRsvpUpdated {
    pub calendar_event_rsvp: CalendarEventRsvp,
    pub server_id: Id<ServerMarker>,
}
