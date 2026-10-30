//! Session-sharing logic related to the terminal view.

#[cfg(feature = "agent_runtime")]
pub(in crate::terminal::view) mod adapter;
pub(crate) mod cloud_conversation_continuation;
mod conversation_ended_tombstone_view;
#[cfg(feature = "agent_runtime")]
pub(in crate::terminal::view) mod sharer;
#[cfg(all(test, feature = "agent_runtime"))]
pub mod test_utils;
#[cfg(feature = "agent_runtime")]
mod view_impl;
#[cfg(feature = "agent_runtime")]
mod viewer;

#[cfg(feature = "agent_runtime")]
pub(in crate::terminal::view) use adapter::Adapter as SharedSessionAdapter;
#[cfg(feature = "agent_runtime")]
pub(in crate::terminal::view) use conversation_ended_tombstone_view::{
    ConversationEndedTombstoneEvent, ConversationEndedTombstoneView,
};
#[cfg(feature = "agent_runtime")]
pub(in crate::terminal::view) use viewer::Viewer;
