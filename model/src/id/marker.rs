//! Markers for various resource IDs.
//!
//! These markers perform no logical action themselves, instead they are used for type-checking.

/// A marker for channel IDs.
///
/// Channel IDs on Guilded are unique UUIDs (for example, `00000000-0000-0000-0000-000000000000`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ChannelMarker;

/// A marker for document IDs.
///
/// Document IDs on guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct DocumentMarker;

/// A marker for forum thread IDs.
///
/// Forum thread IDs on guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ForumThreadMarker;

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
/// Reaction IDs on guilded are unique numerical IDs (for example, `123456`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ReactionMarker;

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
