//! A chat message embed.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbed {
    author: Option<ChatEmbedAuthor>,
    color: Option<u32>,
    description: Option<String>,
    fields: Option<Vec<ChatEmbedField>>,
    footer: Option<ChatEmbedFooter>,
    image: Option<ChatEmbedImage>,
    thumbnail: Option<ChatEmbedThumbnail>,
    timestamp: Option<Timestamp>,
    title: Option<String>,
    url: Option<String>,
}

impl ChatEmbed {
    pub fn author(&self) -> Option<ChatEmbedAuthor> {
        self.author.clone()
    }

    pub fn color(&self) -> Option<u32> {
        self.color
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn fields(&self) -> Option<Vec<ChatEmbedField>> {
        self.fields.clone()
    }

    pub fn footer(&self) -> Option<ChatEmbedFooter> {
        self.footer.clone()
    }

    pub fn image(&self) -> Option<ChatEmbedImage> {
        self.image.clone()
    }

    pub fn thumbnail(&self) -> Option<ChatEmbedThumbnail> {
        self.thumbnail.clone()
    }

    pub fn timestamp(&self) -> Option<Timestamp> {
        self.timestamp
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedAuthor {
    icon_url: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

impl ChatEmbedAuthor {
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedField {
    inline: Option<bool>,
    name: String,
    value: String,
}

impl ChatEmbedField {
    pub fn inline(&self) -> Option<bool> {
        self.inline
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedFooter {
    icon_url: Option<String>,
    text: String,
}

impl ChatEmbedFooter {
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedImage {
    url: Option<String>,
}

impl ChatEmbedImage {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbedThumbnail {
    url: Option<String>,
}

impl ChatEmbedThumbnail {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}
