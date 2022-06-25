use guilded_model::id::{marker::ServerMarker, Id};
use guilded_model::server::Server;

use crate::client::Client;
use crate::error::Error;
use crate::request::Request;
use crate::response::future::ResponseFuture;
use crate::route::Route;

#[must_use = "requests must be configured and executed"]
pub struct ServerRead<'a> {
    client: &'a Client,
    server_id: Id<ServerMarker>,
}

impl<'a> ServerRead<'a> {
    pub(crate) fn new(client: &'a Client, server_id: Id<ServerMarker>) -> Self {
        Self { client, server_id }
    }

    pub fn finish(self) -> ResponseFuture<Server> {
        let client = self.client;

        match self.try_into() {
            Ok(request) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryInto<Request> for ServerRead<'_> {
    type Error = Error;

    fn try_into(self) -> Result<Request, Self::Error> {
        Ok(Request::builder(&Route::ServerRead {
            server_id: &self.server_id.value().unwrap_alphanumeric(),
        })
        .build())
    }
}
