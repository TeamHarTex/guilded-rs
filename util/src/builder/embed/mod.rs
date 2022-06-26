use guilded_model::messaging::embed::ChatEmbed;

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed"]
pub struct ChatEmbedBuilder(ChatEmbed);
