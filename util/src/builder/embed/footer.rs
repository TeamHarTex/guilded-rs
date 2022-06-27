use guilded_model::messaging::embed::ChatEmbedFooter;
use guilded_validation::embed::EmbedValidationError;

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed footer"]
pub struct ChatEmbedFooterBuilder(ChatEmbedFooter);

impl ChatEmbedFooterBuilder {
    pub fn new(text: impl Into<String>) -> Result<Self, EmbedValidationError> {
        todo!()
    }
}
