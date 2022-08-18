use guilded_model::messaging::embed::ChatEmbedThumbnail;

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed thumbnail"]
pub struct ChatEmbedThumbnailBuilder(ChatEmbedThumbnail);

impl ChatEmbedThumbnailBuilder {
    pub fn new() -> Self {
        Self(ChatEmbedThumbnail {
            url: None
        })
    }

    #[must_use = "should be used as part of an embed"]
    pub fn build(self) -> ChatEmbedThumbnail {
        self.0
    }
}
