use guilded_model::messaging::embed::ChatEmbedField;
use guilded_validation::embed::field::{self, ChatEmbedFieldValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed field"]
pub struct ChatEmbedFieldBuilder(ChatEmbedField);

impl ChatEmbedFieldBuilder {
    pub fn new(name: String, value: String) -> Result<Self, ChatEmbedFieldValidationError> {
        field::validate_field_name_length(&name)?;
        field::validate_field_value_length(&value)?;

        Ok(Self(ChatEmbedField {
            inline: None,
            name,
            value,
        }))
    }

    #[must_use = "should be used as part of an embed"]
    pub fn build(self) -> ChatEmbedField {
        self.0
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.0.inline.replace(inline);
        self
    }
}
