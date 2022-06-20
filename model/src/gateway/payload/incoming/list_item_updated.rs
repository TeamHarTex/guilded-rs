//! The ListItemUpdated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::list::ListItem;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemUpdated {
    list_item: ListItem,
    server_id: Id<ServerMarker>,
}