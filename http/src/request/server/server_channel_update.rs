use guilded_model::id::{marker::ChannelMarker, Id};
use guilded_validation::channel::{self, ChannelValidationError};
use serde::Serialize;

use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ServerChannelUpdate<'a> {
    client: &'a Client,
    channel_id: Id<ChannelMarker>,
    fields: ServerChannelUpdateFields<'a>,
}

impl<'a> ServerChannelUpdate<'a> {
    pub(crate) fn new(client: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            client,
            channel_id,
            fields: ServerChannelUpdateFields {
                name: None,
                is_public: None,
                topic: None,
            },
        }
    }

    pub fn name(mut self, name: &'a str) -> Result<Self, ChannelValidationError> {
        channel::validate_name_length(name)?;

        self.fields.name.replace(name);
        Ok(self)
    }

    pub fn is_public(mut self, is_public: bool) -> Self {
        self.fields.is_public.replace(is_public);
        self
    }

    pub fn topic(mut self, topic: &'a str) -> Result<Self, ChannelValidationError> {
        channel::validate_topic_length(topic)?;

        self.fields.topic.replace(topic);
        Ok(self)
    }
}

impl TryInto<Request> for ServerChannelUpdate<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut request = Request::builder(&Route::ServerChannelUpdate {
            channel_id: &self.channel_id.value().unwrap_alphanumeric(),
        });
        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerChannelUpdateFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
}
