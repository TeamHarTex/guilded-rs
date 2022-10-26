use guilded_model::messaging::embed::ChatEmbedAuthor;
use guilded_validation::embed::author::{self, ChatEmbedAuthorValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed thumbnail"]
pub struct ChatEmbedAuthorBuilder(ChatEmbedAuthor);

impl ChatEmbedAuthorBuilder {
    pub fn new() -> Self {
        Self(ChatEmbedAuthor {
            icon_url: None,
            name: None,
            url: None,
        })
    }

    #[must_use = "should be used as part of an embed"]
    pub fn build(self) -> ChatEmbedAuthor {
        self.0
    }

    pub fn icon_url(
        mut self,
        icon_url: impl Into<String>,
    ) -> Result<Self, ChatEmbedAuthorValidationError> {
        let icon_url = icon_url.into();
        author::validate_author_icon_url_length(&icon_url)?;

        self.0.icon_url.replace(icon_url);
        Ok(self)
    }

    pub fn name(mut self, name: impl Into<String>) -> Result<Self, ChatEmbedAuthorValidationError> {
        let name = name.into();
        author::validate_author_name_length(&name)?;

        self.0.name.replace(name);
        Ok(self)
    }

    pub fn url(mut self, url: impl Into<String>) -> Result<Self, ChatEmbedAuthorValidationError> {
        let url = url.into();
        author::validate_author_url_length(&url)?;

        self.0.url.replace(url);
        Ok(self)
    }
}
