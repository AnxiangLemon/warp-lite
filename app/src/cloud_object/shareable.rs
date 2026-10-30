use chrono::{DateTime, Local};
use session_sharing_protocol::common::SessionId;
use warp_core::channel::ChannelState;
use warpui::{AppContext, SingletonEntity, WeakViewHandle};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::BlocklistAIHistoryModel;
#[cfg(not(feature = "warp_lite"))]
use crate::cloud_object::model::persistence::CloudModel;
#[cfg(not(feature = "warp_lite"))]
use crate::server::ids::ServerId;
use crate::terminal::TerminalView;
use crate::terminal::shared_session::join_link;

/// Identifier for an object that's shareable via the Warp Drive ACL model. Not all sharing in Warp
/// is _currently_ tied into this model (e.g. block sharing).
#[derive(Debug, Clone)]
pub enum ShareableObject {
    /// A shareable Warp Drive object.
    #[cfg(not(feature = "warp_lite"))]
    WarpDriveObject(ServerId),
    /// A shared terminal session. Shared sessions are identified by the participating terminal
    /// pane.
    Session {
        handle: WeakViewHandle<TerminalView>,
        session_id: SessionId,
        started_at: DateTime<Local>,
    },
    /// An AI conversation.
    AIConversation(AIConversationId),
}

impl ShareableObject {
    /// The canonical link to this object.
    pub fn link(&self, app: &AppContext) -> Option<String> {
        match self {
            #[cfg(not(feature = "warp_lite"))]
            ShareableObject::WarpDriveObject(id) => CloudModel::as_ref(app)
                .get_by_uid(&id.uid())
                .and_then(|object| object.object_link()),
            ShareableObject::Session { session_id, .. } => Some(join_link(session_id)),
            ShareableObject::AIConversation(id) => {
                BlocklistAIHistoryModel::as_ref(app)
                    .get_server_conversation_metadata(id)
                    .map(|m| {
                        format!(
                            "{}/conversation/{}",
                            ChannelState::server_root_url(),
                            m.server_conversation_token.as_str()
                        )
                    })
            }
        }
    }
}
