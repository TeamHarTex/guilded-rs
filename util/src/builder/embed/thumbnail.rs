use guilded_model::messaging::embed::ChatEmbedThumbnail;
use guilded_validation::embed::thumbnail::{self, ChatEmbedThumbnailValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed thumbnail"]
pub struct ChatEmbedThumbnailBuilder(ChatEmbedThumbnail);

impl ChatEmbedThumbnailBuilder {
    pub fn new() -> Self {
        Self(ChatEmbedThumbnail { url: None })
    }

    #[must_use = "should be used as part of an embed"]
    pub fn build(self) -> ChatEmbedThumbnail {
        self.0
    }

    pub fn url(
        mut self,
        url: impl Into<String>,
    ) -> Result<Self, ChatEmbedThumbnailValidationError> {
        let url = url.into();
        thumbnail::validate_thumbnail_length(&url)?;

        self.0.url.replace(url);
        Ok(self)
    }
}
