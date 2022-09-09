use guilded_model::datetime::Timestamp;
use guilded_model::messaging::embed::{
    ChatEmbed, ChatEmbedFooter, ChatEmbedImage, ChatEmbedThumbnail,
};
use guilded_validation::embed::footer::ChatEmbedFooterValidationError;
use guilded_validation::embed::thumbnail::ChatEmbedThumbnailValidationError;
use guilded_validation::embed::{self, ChatEmbedValidationError};

pub mod footer;
pub mod thumbnail;

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed"]
pub struct ChatEmbedBuilder(ChatEmbed);

impl ChatEmbedBuilder {
    pub fn new() -> Self {
        Self(ChatEmbed {
            author: None,
            color: None,
            description: None,
            fields: None,
            footer: None,
            image: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
        })
    }

    #[must_use = "should be used as part of something like a message"]
    pub fn build(self) -> ChatEmbed {
        self.0
    }

    pub fn color(mut self, color: u32) -> Result<Self, ChatEmbedValidationError> {
        embed::validate_color_range(color)?;

        self.0.color.replace(color);
        Ok(self)
    }

    pub fn description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, EmbedValidationError> {
        let description = description.into();
        embed::validate_description_length(&description)?;

        self.0.description.replace(description);
        Ok(self)
    }

    pub fn footer(
        mut self,
        footer: ChatEmbedFooter,
    ) -> Result<Self, ChatEmbedFooterValidationError> {
        if let Some(icon_url) = &footer.icon_url {
            embed::footer::validate_footer_icon_url_length(icon_url)?;
        }

        embed::footer::validate_footer_text_length(&footer.text)?;

        self.0.footer.replace(footer);
        Ok(self)
    }

    pub fn image(mut self, image: ChatEmbedImage) -> Self {

    }

    pub fn thumbnail(
        mut self,
        thumbnail: ChatEmbedThumbnail,
    ) -> Result<Self, ChatEmbedThumbnailValidationError> {
        if let Some(url) = &thumbnail.url {
            embed::thumbnail::validate_thumbnail_length(url)?;
        }

        self.0.thumbnail.replace(thumbnail);
        Ok(self)
    }

    pub fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.0.timestamp.replace(timestamp);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Result<Self, ChatEmbedValidationError> {
        let title = title.into();
        embed::validate_title_length(&title)?;

        self.0.title.replace(title);
        Ok(self)
    }
}
