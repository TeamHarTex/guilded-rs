//! HTTP Routes of the Guilded API.

use std::fmt::{Display, Formatter, Result as FmtResult};

use guilded_model::datetime::Timestamp;

use crate::request::Method;

pub enum Route<'a> {
    CalendarEventCreate {
        channel_id: &'a str,
    },
    CalendarEventDelete {
        calendar_event_id: u32,
        channel_id: &'a str,
    },
    CalendarEventRead {
        calendar_event_id: u32,
        channel_id: &'a str,
    },
    CalendarEventReadMany {
        after: Option<Timestamp>,
        before: Option<Timestamp>,
        channel_id: &'a str,
        limit: Option<u64>,
    },
    CalendarEventRsvpDelete {
        calendar_event_id: u32,
        channel_id: &'a str,
        user_id: &'a str,
    },
    CalendarEventRsvpRead {
        calendar_event_id: u32,
        channel_id: &'a str,
        user_id: &'a str,
    },
    CalendarEventRsvpReadMany {
        calendar_event_id: u32,
        channel_id: &'a str,
    },
    CalendarEventRsvpUpdate {
        calendar_event_id: u32,
        channel_id: &'a str,
        user_id: &'a str,
    },
    CalendarEventUpdate {
        calendar_event_id: u32,
        channel_id: &'a str,
    },
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
    ContentReactionCreate {
        channel_id: &'a str,
        content_id: &'a str,
        emote_id: u32,
    },
    ContentReactionDelete {
        channel_id: &'a str,
        content_id: &'a str,
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
    ForumTopicCommentCreate {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicCommentDelete {
        channel_id: &'a str,
        forum_topic_comment_id: u32,
        forum_topic_id: u32,
    },
    ForumTopicCommentRead {
        channel_id: &'a str,
        forum_topic_comment_id: u32,
        forum_topic_id: u32,
    },
    ForumTopicCommentReadMany {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicCommentUpdate {
        channel_id: &'a str,
        forum_topic_comment_id: u32,
        forum_topic_id: u32,
    },
    ForumTopicCreate {
        channel_id: &'a str,
    },
    ForumTopicDelete {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicLock {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicPin {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicRead {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicReadMany {
        channel_id: &'a str,
    },
    ForumTopicUnlock {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicUnpin {
        channel_id: &'a str,
        forum_topic_id: u32,
    },
    ForumTopicUpdate {
        channel_id: &'a str,
        forum_topic_id: u32,
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
    ServerChannelCreate,
    ServerChannelDelete {
        channel_id: &'a str,
    },
    ServerChannelRead {
        channel_id: &'a str,
    },
    ServerChannelUpdate {
        channel_id: &'a str,
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
    UserRead {
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
    },
    WebhookUpdate {
        server_id: &'a str,
        webhook_id: &'a str,
    },
}

impl<'a> Route<'a> {
    pub fn method(&self) -> Method {
        match self {
            Self::CalendarEventRead { .. }
            | Self::CalendarEventReadMany { .. }
            | Self::CalendarEventRsvpRead { .. }
            | Self::CalendarEventRsvpReadMany { .. }
            | Self::ChannelMessageRead { .. }
            | Self::ChannelMessageReadMany { .. }
            | Self::DocRead { .. }
            | Self::DocReadMany { .. }
            | Self::ForumTopicCommentRead { .. }
            | Self::ForumTopicCommentReadMany { .. }
            | Self::ForumTopicRead { .. }
            | Self::ForumTopicReadMany { .. }
            | Self::ListItemRead { .. }
            | Self::ListItemReadMany { .. }
            | Self::MemberSocialLinkRead { .. }
            | Self::RoleMembershipReadMany { .. }
            | Self::ServerChannelRead { .. }
            | Self::ServerMemberBanRead { .. }
            | Self::ServerMemberBanReadMany { .. }
            | Self::ServerMemberRead { .. }
            | Self::ServerMemberReadMany { .. }
            | Self::ServerRead { .. }
            | Self::UserRead { .. }
            | Self::WebhookRead { .. }
            | Self::WebhookReadMany { .. } => Method::Get,
            Self::CalendarEventDelete { .. }
            | Self::CalendarEventRsvpDelete { .. }
            | Self::ChannelMessageDelete { .. }
            | Self::ContentReactionDelete { .. }
            | Self::DocDelete { .. }
            | Self::ForumTopicCommentDelete { .. }
            | Self::ForumTopicDelete { .. }
            | Self::ForumTopicUnlock { .. }
            | Self::ForumTopicUnpin { .. }
            | Self::GroupMembershipDelete { .. }
            | Self::ListItemCompleteDelete { .. }
            | Self::ListItemDelete { .. }
            | Self::MemberNicknameDelete { .. }
            | Self::RoleMembershipDelete { .. }
            | Self::ServerChannelDelete { .. }
            | Self::ServerMemberDelete { .. }
            | Self::ServerMemberBanDelete { .. }
            | Self::WebhookDelete { .. } => Method::Delete,
            Self::CalendarEventUpdate { .. }
            | Self::ForumTopicCommentUpdate { .. }
            | Self::ForumTopicUpdate { .. }
            | Self::ServerChannelUpdate { .. } => Method::Patch,
            Self::CalendarEventCreate { .. }
            | Self::ChannelMessageCreate { .. }
            | Self::ContentReactionCreate { .. }
            | Self::DocCreate { .. }
            | Self::ForumTopicCommentCreate { .. }
            | Self::ForumTopicCreate { .. }
            | Self::GroupMembershipCreate { .. }
            | Self::ListItemCreate { .. }
            | Self::ListItemCompleteCreate { .. }
            | Self::RoleMembershipCreate { .. }
            | Self::ServerChannelCreate
            | Self::ServerMemberBanCreate { .. }
            | Self::ServerXpForRoleCreate { .. }
            | Self::ServerXpForUserCreate { .. }
            | Self::WebhookCreate { .. } => Method::Post,
            Self::CalendarEventRsvpUpdate { .. }
            | Self::ChannelMessageUpdate { .. }
            | Self::DocUpdate { .. }
            | Self::ForumTopicLock { .. }
            | Self::ForumTopicPin { .. }
            | Self::ListItemUpdate { .. }
            | Self::MemberNicknameUpdate { .. }
            | Self::WebhookUpdate { .. } => Method::Put,
        }
    }
}

impl Display for Route<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::CalendarEventCreate { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/events")
            }
            Self::CalendarEventDelete {
                calendar_event_id,
                channel_id,
            }
            | Self::CalendarEventRead {
                calendar_event_id,
                channel_id,
            }
            | Self::CalendarEventUpdate {
                calendar_event_id,
                channel_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/events/")?;
                Display::fmt(calendar_event_id, f)
            }
            Self::CalendarEventReadMany {
                after,
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/events?")?;

                if let Some(after) = after {
                    f.write_str("after=")?;
                    Display::fmt(&after.display(), f)?;
                }

                if let Some(before) = before {
                    f.write_str("&before=")?;
                    Display::fmt(&before.display(), f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Self::CalendarEventRsvpRead {
                calendar_event_id,
                channel_id,
                user_id,
            }
            | Self::CalendarEventRsvpDelete {
                calendar_event_id,
                channel_id,
                user_id,
            }
            | Self::CalendarEventRsvpUpdate {
                calendar_event_id,
                channel_id,
                user_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/events/")?;
                Display::fmt(calendar_event_id, f)?;
                f.write_str("/rsvps/")?;

                Display::fmt(user_id, f)
            }
            Self::CalendarEventRsvpReadMany {
                calendar_event_id,
                channel_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/events/")?;
                Display::fmt(calendar_event_id, f)?;

                f.write_str("/rsvps")
            }
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
                    Display::fmt(&after.display(), f)?;
                }

                if let Some(before) = before {
                    f.write_str("&before=")?;
                    Display::fmt(&before.display(), f)?;
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
            Self::ContentReactionCreate {
                channel_id,
                content_id,
                emote_id,
            }
            | Self::ContentReactionDelete {
                channel_id,
                content_id,
                emote_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/content/")?;
                Display::fmt(content_id, f)?;
                f.write_str("/emotes/")?;
                Display::fmt(emote_id, f)
            }
            Self::DocCreate { channel_id } | Self::DocReadMany { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/docs")
            }
            Self::DocDelete { channel_id, doc_id }
            | Self::DocRead { channel_id, doc_id }
            | Self::DocUpdate { channel_id, doc_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/docs/")?;
                Display::fmt(doc_id, f)
            }
            Self::ForumTopicCommentCreate {
                channel_id,
                forum_topic_id,
            }
            | Self::ForumTopicCommentReadMany {
                channel_id,
                forum_topic_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/topics/")?;
                Display::fmt(forum_topic_id, f)?;
                f.write_str("/comments")
            }
            Self::ForumTopicCommentDelete {
                channel_id,
                forum_topic_comment_id,
                forum_topic_id,
            }
            | Self::ForumTopicCommentRead {
                channel_id,
                forum_topic_comment_id,
                forum_topic_id,
            }
            | Self::ForumTopicCommentUpdate {
                channel_id,
                forum_topic_comment_id,
                forum_topic_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/topics/")?;
                Display::fmt(forum_topic_id, f)?;
                f.write_str("/comments/")?;
                Display::fmt(forum_topic_comment_id, f)
            }
            Self::ForumTopicCreate { channel_id } | Self::ForumTopicReadMany { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/topics")
            }
            Self::ForumTopicDelete {
                channel_id,
                forum_topic_id,
            }
            | Self::ForumTopicRead {
                channel_id,
                forum_topic_id,
            }
            | Self::ForumTopicUpdate {
                channel_id,
                forum_topic_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/topics/")?;
                Display::fmt(forum_topic_id, f)
            }
            Self::ForumTopicLock {
                channel_id,
                forum_topic_id,
            }
            | Self::ForumTopicUnlock {
                channel_id,
                forum_topic_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/topics/")?;
                Display::fmt(forum_topic_id, f)?;
                f.write_str("/lock")
            }
            Self::ForumTopicPin {
                channel_id,
                forum_topic_id,
            }
            | Self::ForumTopicUnpin {
                channel_id,
                forum_topic_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/topics/")?;
                Display::fmt(forum_topic_id, f)?;
                f.write_str("/pin")
            }
            Self::GroupMembershipCreate { group_id, user_id }
            | Self::GroupMembershipDelete { group_id, user_id } => {
                f.write_str("groups/")?;
                Display::fmt(group_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)
            }
            Self::ListItemCreate { channel_id } | Self::ListItemReadMany { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/items")
            }
            Self::ListItemCompleteCreate {
                channel_id,
                list_item_id,
            }
            | Self::ListItemCompleteDelete {
                channel_id,
                list_item_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/items/")?;
                Display::fmt(list_item_id, f)?;
                f.write_str("/complete")
            }
            Self::ListItemDelete {
                channel_id,
                list_item_id,
            }
            | Self::ListItemRead {
                channel_id,
                list_item_id,
            }
            | Self::ListItemUpdate {
                channel_id,
                list_item_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/items/")?;
                Display::fmt(list_item_id, f)
            }
            Self::MemberNicknameDelete { server_id, user_id }
            | Self::MemberNicknameUpdate { server_id, user_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/nickname")
            }
            Self::MemberSocialLinkRead {
                server_id,
                user_id,
                r#type,
            } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/social-links/")?;
                Display::fmt(r#type, f)
            }
            Self::RoleMembershipCreate {
                role_id,
                server_id,
                user_id,
            }
            | Self::RoleMembershipDelete {
                role_id,
                server_id,
                user_id,
            } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/roles/")?;
                Display::fmt(role_id, f)
            }
            Self::RoleMembershipReadMany { server_id, user_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/roles")
            }
            Self::ServerChannelCreate => f.write_str("channels"),
            Self::ServerChannelDelete { channel_id }
            | Self::ServerChannelRead { channel_id }
            | Self::ServerChannelUpdate { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)
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
            Self::ServerRead { server_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)
            }
            Self::ServerXpForRoleCreate { server_id, role_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/roles/")?;
                Display::fmt(role_id, f)?;
                f.write_str("/xp")
            }
            Self::ServerXpForUserCreate { server_id, user_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/xp")
            }
            Self::UserRead { user_id } => {
                f.write_str("users/")?;
                Display::fmt(user_id)
            }
            Self::WebhookCreate { server_id } | Self::WebhookReadMany { server_id } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/webhooks")
            }
            Self::WebhookDelete {
                server_id,
                webhook_id,
            }
            | Self::WebhookRead {
                server_id,
                webhook_id,
            }
            | Self::WebhookUpdate {
                server_id,
                webhook_id,
            } => {
                f.write_str("servers/")?;
                Display::fmt(server_id, f)?;
                f.write_str("/webhooks/")?;
                Display::fmt(webhook_id, f)
            }
        }
    }
}
