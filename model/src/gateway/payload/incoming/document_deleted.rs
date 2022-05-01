//! The DocDeleted event.

use serde::{Deserialize, Serialize};

use crate::document::Document;
use crate::id::{marker::ServerMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDeleted {
    doc: Document,
    server_id: Id<ServerMarker>,
}
