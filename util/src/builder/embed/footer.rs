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
}
