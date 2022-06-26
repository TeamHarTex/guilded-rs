use guilded_model::messaging::embed::ChatEmbed;
use guilded_validation::embed::{self, EmbedValidationError};

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

    pub fn title(mut self, title: impl Into<String>) -> Result<Self, EmbedValidationError> {
        let title = title.into();
        embed::validate_title_length(&title)?;

        self.0.title.replace(title);
        Ok(self)
    }
}
