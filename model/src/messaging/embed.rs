//! A chat message embed.

use serde::{Deserialize, Serialize};

use crate::datetime::Timestamp;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageEmbed {
    author: Option<ChatMessageEmbedAuthor>,
    color: Option<u32>,
    description: Option<String>,
    fields: Option<Vec<ChatMessageEmbedField>>,
    footer: Option<ChatMessageEmbedFooter>,
    image: Option<ChatMessageEmbedImage>,
    thumbnail: Option<ChatMessageEmbedThumbnail>,
    timestamp: Option<Timestamp>, // FIXME: should belong in footer object
    title: Option<String>,
    url: Option<String>,
}

impl ChatMessageEmbed {
    pub fn author(&self) -> Option<ChatMessageEmbedAuthor> {
        self.author.clone()
    }

    pub fn color(&self) -> Option<u32> {
        self.color
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn fields(&self) -> Option<Vec<ChatMessageEmbedField>> {
        self.fields.clone()
    }

    pub fn footer(&self) -> Option<ChatMessageEmbedFooter> {
        self.footer.clone()
    }

    pub fn image(&self) -> Option<ChatMessageEmbedImage> {
        self.image.clone()
    }

    pub fn thumbnail(&self) -> Option<ChatMessageEmbedThumbnail> {
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
pub struct ChatMessageEmbedAuthor {
    icon_url: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

impl ChatMessageEmbedAuthor {
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
pub struct ChatMessageEmbedField {
    inline: Option<bool>,
    name: String,
    value: String,
}

impl ChatMessageEmbedField {
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
pub struct ChatMessageEmbedFooter {
    icon_url: Option<String>,
    text: String,
}

impl ChatMessageEmbedFooter {
    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageEmbedImage {
    url: Option<String>,
}

impl ChatMessageEmbedImage {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageEmbedThumbnail {
    url: Option<String>,
}

impl ChatMessageEmbedThumbnail {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}
