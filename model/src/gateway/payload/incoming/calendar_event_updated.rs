//! The CalendarEventUpdated event.

use crate::events::CalendarEvent;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventUpdated {
    calendar_event: CalendarEvent,
    server_id: Id<ServerMarker>,
}

impl CalendarEventUpdated {
    pub fn calendar_event(&self) -> CalendarEvent {
        self.calendar_event.clone()
    }

    pub fn server_id(&self) -> Id<ServerMarker> {
        self.server_id.clone()
    }
}