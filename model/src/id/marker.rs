//! Markers for various resource IDs.
//!
//! These markers perform no logical action themselves, instead they are used for type-checking.

/// A marker for channel IDs.
///
/// Channel IDs on Guilded are unique UUIDs (for example, `00000000-0000-0000-0000-000000000000`).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ChannelMarker;

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
