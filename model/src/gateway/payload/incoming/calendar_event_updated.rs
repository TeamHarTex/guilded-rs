//! The CalendarEventUpdated event.

use serde::{Deserialize, Serialize};

use crate::event::CalendarEvent;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventUpdated {
    pub calendar_event: CalendarEvent,
    pub server_id: Id<ServerMarker>,
}

