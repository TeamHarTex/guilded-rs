//! HTTP Routes of the Guilded API.

use std::fmt::{Display, Formatter, Result as FmtResult, Write};

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
        after: Option<Timestamp>,
        before: Option<Timestamp>,
        channel_id: &'a str,
        include_private: Option<bool>,
        limit: Option<u64>,
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
    ForumTopicCreate {
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
        r#type: &'a str,
        user_id: &'a str,
    },
    RoleMembershipCreate {
        role_id: u32,
        server_id: &'a str,
        user_id: &'a str,
    },
    RoleMembershipDelete {
        role_id: u32,
        server_id: &'a str,
        user_id: &'a str,
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
        channel_id: &'a str,
        server_id: &'a str,
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
            | Self::ForumTopicCreate { .. }
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

impl Display for Route<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ChannelMessageCreate { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/messages")
            }
            Self::ChannelMessageDelete {
                channel_id,
                message_id,
            }
            | Self::ChannelMessageRead {
                channel_id,
                message_id,
            }
            | Self::ChannelMessageUpdate {
                channel_id,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;

                Display::fmt(message_id, f)
            }
            Self::ChannelMessageReadMany {
                after,
                before,
                channel_id,
                include_private,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages?")?;

                if let Some(after) = after {
                    f.write_str("after=")?;
                    Display::fmt(after, f)?;
                }

                if let Some(before) = before {
                    f.write_str("&before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(include_private) = include_private {
                    f.write_str("&includePrivate=")?;
                    Display::fmt(include_private, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Self::ForumTopicCreate { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id)?;
                f.write_str("/topics")
            }
            Self::ListItemCreate { channel_id }
            | Self::ListItemReadMany { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/items")
            }
            Self::ListItemCompleteCreate { channel_id, list_item_id }
            | Self::ListItemCompleteDelete { channel_id, list_item_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/items/")?;
                Display::fmt(list_item_id)?;
                f.write_str("/complete")
            }
            Self::ListItemDelete { channel_id, list_item_id }
            | Self::ListItemRead { channel_id, list_item_id }
            | Self::ListItemUpdate { channel_id, list_item_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/items/")?;
                Display::fmt(list_item_id)
            }
            Self::MemberNicknameDelete { server_id, user_id }
            | Self::MemberNicknameUpdate { server_id, user_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/nickname")
            }
            Self::ServerMemberBanCreate { server_id, user_id }
            | Self::ServerMemberBanDelete { server_id, user_id }
            | Self::ServerMemberBanRead { server_id, user_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/bans/")?;
                Display::fmt(user_id, f)
            }
            Self::ServerMemberBanReadMany { server_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/bans")
            }
            Self::ServerMemberDelete { server_id, user_id }
            | Self::ServerMemberRead { server_id, user_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)
            }
            Self::ServerMemberReadMany { server_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members")
            }
            _ => todo!(),
        }
    }
}
