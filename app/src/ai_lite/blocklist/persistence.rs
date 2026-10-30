use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent::AIAgentExchangeId;
use crate::ai::llms::LLMId;

#[derive(Debug, Clone, Deserialize)]
pub struct PersistedAIInput {
    pub(crate) exchange_id: AIAgentExchangeId,
    pub(crate) conversation_id: AIConversationId,
    pub(crate) start_ts: DateTime<Local>,
    pub(crate) inputs: Vec<PersistedAIInputType>,
    pub(crate) output_status: AIQueryHistoryOutputStatus,
    pub(crate) working_directory: Option<String>,
    pub(crate) model_id: LLMId,
    pub(crate) coding_model_id: LLMId,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PersistedAIInputType {
    Query {
        text: String,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIQueryHistoryOutputStatus {
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerializedBlockListItem {
    pub blocks: Vec<crate::terminal::model::block::SerializedBlock>,
}

pub fn maybe_build_ai_query_upsert_event(
    _input: &PersistedAIInput,
) -> Option<crate::persistence::ModelEvent> {
    None
}
