//! A chat message embed.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbed {
    pub author: Option<ChatEmbedAuthor>,
    pub color: Option<u32>,
    pub description: Option<String>,
    pub fields: Option<Vec<ChatEmbedField>>,
    pub footer: Option<ChatEmbedFooter>,
    pub image: Option<ChatEmbedImage>,
    pub thumbnail: Option<ChatEmbedThumbnail>,
    pub timestamp: Option<Timestamp>,
    pub title: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedAuthor {
    pub icon_url: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedField {
    pub inline: Option<bool>,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedFooter {
    pub icon_url: Option<String>,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedImage {
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedThumbnail {
    pub url: Option<String>,
}
