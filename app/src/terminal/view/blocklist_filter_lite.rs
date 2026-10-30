//! Lite stub for blocklist filtering utilities.

use crate::ai::agent::AIAgentExchange;
use crate::ai::agent::conversation::AIConversation;
use crate::ai::agent::task::Task;

pub fn should_show_task_in_blocklist(_task: &Task) -> bool {
    false
}

pub(crate) fn conversation_would_render_in_blocklist(_conversation: &AIConversation) -> bool {
    false
}

pub(crate) fn exchanges_for_blocklist(_conversation: &AIConversation) -> Vec<&AIAgentExchange> {
    vec![]
}
