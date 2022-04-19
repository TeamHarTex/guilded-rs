//! HTTP Routes of the Guilded API.

use guilded_model::datetime::Timestamp;

pub enum Route<'a> {
    ChannelMessageCreate {
        channel_id: &'a str,
    },
    ChannelMessageDelete {
        channel_id: &'a str,
        message_id: &'a str,
    },
    ChannelMessageRead {
        channel_id: &'a str,
        message_id: &'a str,
    },
    ChannelMessageReadMany {
        channel_id: &'a str,
        before: Option<Timestamp>,
        after: Option<Timestamp>,
        limit: Option<u64>,
        include_private: Option<bool>,
    },
    ChannelMessageUpdate {
        channel_id: &'a str,
        message_id: &'a str,
    },
    ContentReactionCreate {
        channel_id: &'a str,
        content_id: String,
        emote_id: u32,
    },
    DocCreate {
        channel_id: &'a str,
    },
    DocDelete {
        channel_id: &'a str,
        doc_id: u32,
    },
    DocRead {
        channel_id: &'a str,
        doc_id: u32,
    },
    DocReadMany {
        channel_id: &'a str,
    },
    DocUpdate {
        channel_id: &'a str,
        doc_id: u32,
    },
    ForumThreadCreate {
        channel_id: &'a str,
    },
    GroupMembershipCreate {
        group_id: &'a str,
        user_id: &'a str,
    },
    GroupMembershipDelete {
        group_id: &'a str,
        user_id: &'a str,
    },
    ListItemCreate {
        channel_id: &'a str,
    },
    ListItemDelete {
        channel_id: &'a str,
        list_item_id: &'a str,
    },
    ListItemRead {
        channel_id: &'a str,
        list_item_id: &'a str,
    },
    ListItemReadMany {
        channel_id: &'a str,
    },
    ListItemUpdate {
        channel_id: &'a str,
        list_item_id: &'a str,
    },
    MemberNicknameDelete {
        server_id: &'a str,
        user_id: &'a str,
    },
    MemberNicknameUpdate {
        server_id: &'a str,
        user_id: &'a str,
    },
    MemberSocialLinkRead {
        server_id: &'a str,
        user_id: &'a str,
        r#type: &'a str,
    },
    RoleMembershipCreate {
        server_id: &'a str,
        user_id: &'a str,
        role_id: u32,
    },
    RoleMembershipDelete {
        server_id: &'a str,
        user_id: &'a str,
        role_id: u32,
    },
    RoleMembershipReadMany {
        server_id: &'a str,
        user_id: &'a str,
    },
    ServerMemberBanCreate {
        server_id: &'a str,
        user_id: &'a str,
    },
    ServerMemberBanDelete {
        server_id: &'a str,
        user_id: &'a str,
    },
    ServerMemberBanRead {
        server_id: &'a str,
        user_id: &'a str,
    },
    ServerMemberBanReadMany {
        server_id: &'a str,
    },
    ServerMemberDelete {
        server_id: &'a str,
        user_id: &'a str,
    },
    ServerMemberRead {
        server_id: &'a str,
        user_id: &'a str,
    },
    ServerMemberReadMany {
        server_id: &'a str,
    },
    ServerXpForRoleCreate {
        server_id: &'a str,
        role_id: u32,
    },
    ServerXpForUserCreate {
        server_id: &'a str,
        user_id: &'a str,
    },
    WebhookCreate {
        server_id: &'a str,
    },
    WebhookDelete {
        server_id: &'a str,
        webhook_id: &'a str,
    },
    WebhookRead {
        server_id: &'a str,
        webhook_id: &'a str,
    },
    WebhookReadMany {
        server_id: &'a str,
        channel_id: &'a str,
    },
    WebhookUpdate {
        server_id: &'a str,
        webhook_id: &'a str,
    },
}
