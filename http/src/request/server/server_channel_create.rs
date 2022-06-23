use crate::client::Client;

use crate::error::Error;
use crate::request::Request;
use guilded_model::channel::{ServerChannel, ServerChannelType};
use guilded_model::id::{
    marker::{CategoryMarker, GroupMarker, ServerMarker},
    Id,
};
use guilded_validation::channel::{self, ChannelValidationError};
use serde::Serialize;

use crate::response::future::ResponseFuture;
use crate::route::Route;

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
                category_id: None,
                group_id: None,
                name,
                is_public: None,
                server_id: None,
                topic: None,
                r#type,
            },
        })
    }

    pub fn category_id(mut self, category_id: Id<CategoryMarker>) -> Self {
        self.fields.category_id.replace(category_id);
        self
    }

    pub fn group_id(mut self, group_id: Id<GroupMarker>) -> Self {
        self.fields.group_id.replace(group_id);
        self
    }

    pub fn is_public(mut self, is_public: bool) -> Self {
        self.fields.is_public.replace(is_public);
        self
    }

    pub fn server_id(mut self, server_id: Id<ServerMarker>) -> Self {
        self.fields.server_id.replace(server_id);
        self
    }

    pub fn topic(mut self, topic: &'a str) -> Result<Self, ChannelValidationError> {
        channel::validate_topic_length(topic)?;

        self.fields.topic.replace(topic);
        Ok(self)
    }

    pub fn finish(self) -> ResponseFuture<ServerChannel> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ServerChannelCreate<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        let mut request = Request::builder(&Route::ServerChannelCreate);
        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerChannelCreateFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<Id<CategoryMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<Id<GroupMarker>>,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_id: Option<Id<ServerMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
    r#type: ServerChannelType,
}
