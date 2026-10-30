use warpui::{AppContext, EntityId};

use crate::ai::agent::conversation::AIConversationId;

pub fn delete_conversation(
    _conversation_id: AIConversationId,
    _terminal_view_id: Option<EntityId>,
    _ctx: &mut AppContext,
) {
}

pub fn remove_conversation(
    _conversation_id: AIConversationId,
    _terminal_view_id: EntityId,
    _delete_from_cloud: bool,
    _ctx: &mut AppContext,
) {
}
