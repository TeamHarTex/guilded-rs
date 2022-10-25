//! Markers for various resource IDs.
//!
//! These markers perform no logical action themselves, instead they are used for type-checking.

/// A marker for calendar event IDs.
///
/// Category IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CalendarEventMarker;

/// A marker for category IDs.
///
/// Category IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CategoryMarker;

/// A marker for channel IDs.
///
/// Channel IDs on Guilded are unique UUIDs (for example, `00000000-0000-0000-0000-000000000000`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ChannelMarker;

/// A marker for document IDs.
///
/// Document IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct DocumentMarker;

/// A marker for emote IDs.
///
/// Emote IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct EmoteMarker;

/// A marker for forum thread comment IDs.
///
/// Forum topic comment IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ForumTopicCommentMarker;

/// A marker for forum topic IDs.
///
/// Forum topic IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ForumTopicMarker;

/// A marker for group IDs.
///
/// Group IDs on Guilded are unique 8-character IDs (for example, `Ann6LewA`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct GroupMarker;

/// A marker for list item IDs.
///
/// List items IDs on Guilded are unique UUIDs (for example, `00000000-0000-0000-0000-000000000000`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ListItemMarker;

/// A marker for message IDs.
///
/// Message IDs on Guilded are unique UUIDs (for example, `00000000-0000-0000-0000-000000000000`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct MessageMarker;

/// A marker for user IDs.
///
/// User IDs on Guilded are unique 8-character IDs (for example, `Ann6LewA`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct UserMarker;

/// A marker for reaction IDs.
///
/// Reaction IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ReactionMarker;

/// A marker for role IDs.
///
/// Reaction IDs on Guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct RoleMarker;

/// A marker for server IDs.
///
/// Server IDs on Guilded are unique 8-character IDs (for example, `Ann6LewA`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ServerMarker;

/// A marker for webhook IDs.
///
/// Webhooks IDs on Guilded are unique UUIDs (for example, `00000000-0000-0000-0000-000000000000`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct WebhookMarker;
