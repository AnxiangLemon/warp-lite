use warpui::{AppContext, Entity, ModelContext, ModelHandle};

use super::agent_view::{AgentViewEntryOrigin, EnterAgentViewError};
use crate::ai::agent::conversation::{AIConversationAutoexecuteMode, AIConversationId};

pub type ConversationSelectionHandle = ModelHandle<Box<dyn ConversationSelection>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PendingQueryState {
    Existing {
        conversation_id: AIConversationId,
    },
    New {
        autoexecute_override: AIConversationAutoexecuteMode,
    },
}

impl Default for PendingQueryState {
    fn default() -> Self {
        Self::New {
            autoexecute_override: AIConversationAutoexecuteMode::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConversationSelectionEvent {
    Changed,
    Activated {
        is_fullscreen: bool,
        origin: AgentViewEntryOrigin,
    },
    Deactivated {
        conversation_id: AIConversationId,
        final_exchange_count: usize,
        is_exit_before_new_entrance: bool,
    },
}

pub trait ConversationSelection: Send {
    fn selected_conversation_id(&self, _app: &AppContext) -> Option<AIConversationId> {
        None
    }

    fn is_conversation_active(&self, _app: &AppContext) -> bool {
        false
    }

    fn is_conversation_fullscreen(&self, _app: &AppContext) -> bool {
        false
    }

    fn select_existing_conversation(
        &mut self,
        _conversation_id: AIConversationId,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ModelContext<Box<dyn ConversationSelection>>,
    ) {
    }

    fn select_new_conversation(
        &mut self,
        _ctx: &mut ModelContext<Box<dyn ConversationSelection>>,
    ) {
    }

    fn pending_query_state(&self, _app: &AppContext) -> PendingQueryState {
        PendingQueryState::default()
    }

    fn enter_agent_view(
        &mut self,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ModelContext<Box<dyn ConversationSelection>>,
    ) -> Result<AIConversationId, EnterAgentViewError> {
        Err(EnterAgentViewError::Unknown)
    }
}
