use guilded_model::messaging::embed::ChatEmbedFooter;
use guilded_validation::embed::footer::{self, EmbedFooterValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed footer"]
pub struct ChatEmbedFooterBuilder(ChatEmbedFooter);

impl ChatEmbedFooterBuilder {
    pub fn new(text: impl Into<String>) -> Result<Self, EmbedFooterValidationError> {
        let text = text.into();
        footer::validate_footer_text_length(&text)?;

        Ok(Self(ChatEmbedFooter {
            icon_url: None,
            text,
        }))
    }

    #[must_use = "should be used as part of an embed"]
    pub fn build(self) -> ChatEmbedFooter {
        self.0
    }

    pub fn icon_url(
        mut self,
        icon_url: impl Into<String>,
    ) -> Result<Self, EmbedFooterValidationError> {
        let icon_url = icon_url.into();
        footer::validate_footer_icon_url_length(&icon_url)?;

        self.0.icon_url.replace(icon_url);
        Ok(self)
    }
}
