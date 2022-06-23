use crate::client::Client;
use guilded_model::channel::ServerChannelType;

use serde::Serialize;

#[must_use = "requests must be configured and executed"]
pub struct ServerChannelCreate<'a> {
    client: &'a Client,
    fields: ServerChannelCreateFields<'a>,
}

#[derive(Serialize)]
struct ServerChannelCreateFields<'a> {
    name: &'a str,
    r#type: ServerChannelType,
}
