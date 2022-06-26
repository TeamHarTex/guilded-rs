//! The ListItemCreated event.

use serde::{Deserialize, Serialize};

use crate::id::{marker::ServerMarker, Id};
use crate::list::ListItem;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemCreated {
    pub list_item: ListItem,
    pub server_id: Id<ServerMarker>,
}
