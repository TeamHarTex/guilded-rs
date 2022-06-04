//! HTTP Routes of the Guilded API.

use guilded_model::datetime::Timestamp;

use crate::request::Method;

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
    ChannelCreate,
    ChannelDelete {
        channel_id: &'a str,
    },
    ChannelRead {
        channel_id: &'a str,
    },
    ChannelUpdate {
        channel_id: &'a str,
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
    ListItemCompleteCreate {
        channel_id: &'a str,
        list_item_id: &'a str,
    },
    ListItemCompleteDelete {
        channel_id: &'a str,
        list_item_id: &'a str,
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
    ServerRead {
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

impl<'a> Route<'a> {
    pub fn method(&self) -> Method {
        match self {
            Self::ChannelMessageRead { .. }
            | Self::ChannelRead { .. }
            | Self::ChannelMessageReadMany { .. }
            | Self::DocRead { .. }
            | Self::DocReadMany { .. }
            | Self::ListItemRead { .. }
            | Self::ListItemReadMany { .. }
            | Self::MemberSocialLinkRead { .. }
            | Self::RoleMembershipReadMany { .. }
            | Self::ServerMemberBanRead { .. }
            | Self::ServerMemberBanReadMany { .. }
            | Self::ServerMemberRead { .. }
            | Self::ServerMemberReadMany { .. }
            | Self::ServerRead { .. }
            | Self::WebhookRead { .. }
            | Self::WebhookReadMany { .. } => Method::Get,
            Self::ChannelDelete { .. }
            | Self::ChannelMessageDelete { .. }
            | Self::DocDelete { .. }
            | Self::GroupMembershipDelete { .. }
            | Self::ListItemCompleteDelete { .. }
            | Self::ListItemDelete { .. }
            | Self::MemberNicknameDelete { .. }
            | Self::RoleMembershipDelete { .. }
            | Self::ServerMemberDelete { .. }
            | Self::ServerMemberBanDelete { .. }
            | Self::WebhookDelete { .. } => Method::Delete,
            Self::ChannelUpdate { .. } => Method::Patch,
            Self::ChannelCreate
            | Self::ChannelMessageCreate { .. }
            | Self::ContentReactionCreate { .. }
            | Self::DocCreate { .. }
            | Self::ForumThreadCreate { .. }
            | Self::GroupMembershipCreate { .. }
            | Self::ListItemCreate { .. }
            | Self::ListItemCompleteCreate { .. }
            | Self::RoleMembershipCreate { .. }
            | Self::ServerMemberBanCreate { .. }
            | Self::ServerXpForRoleCreate { .. }
            | Self::ServerXpForUserCreate { .. }
            | Self::WebhookCreate { .. } => Method::Post,
            Self::ChannelMessageUpdate { .. }
            | Self::DocUpdate { .. }
            | Self::ListItemUpdate { .. }
            | Self::MemberNicknameUpdate { .. }
            | Self::WebhookUpdate { .. } => Method::Put,
        }
    }
}
