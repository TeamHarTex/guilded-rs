use guilded_model::messaging::embed::ChatEmbedImage;
use guilded_validation::embed::image;
use guilded_validation::embed::image::ChatEmbedImageValidationError;

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed image"]
pub struct ChatEmbedImageBuilder(ChatEmbedImage);

impl ChatEmbedImageBuilder {
    pub fn new() -> Self {
        Self(ChatEmbedImage { url: None })
    }

    #[must_use = "should be used as part of an embed"]
    pub fn build(self) -> ChatEmbedImage {
        self.0
    }

    pub fn url(mut self, url: impl Into<String>) -> Result<Self, ChatEmbedImageValidationError> {
        let url = url.into();
        image::validate_image_length(&url)?;

        self.0.url.replace(url);
        Ok(self)
    }
}
