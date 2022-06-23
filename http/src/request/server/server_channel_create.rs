use crate::client::Client;

use guilded_model::channel::ServerChannelType;
use guilded_validation::channel::{self, ChannelValidationError};
use serde::Serialize;

#[must_use = "requests must be configured and executed"]
pub struct ServerChannelCreate<'a> {
    client: &'a Client,
    fields: ServerChannelCreateFields<'a>,
}

impl<'a> ServerChannelCreate<'a> {
    pub(crate) fn new(
        client: &'a Client,
        name: &'a str,
        r#type: ServerChannelType,
    ) -> Result<Self, ChannelValidationError> {
        channel::validate_name_length(name)?;

        Ok(Self {
            client,
            fields: ServerChannelCreateFields {
                name,
                topic: None,
                r#type,
            }
        })
    }
}

#[derive(Serialize)]
struct ServerChannelCreateFields<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
    r#type: ServerChannelType,
}
