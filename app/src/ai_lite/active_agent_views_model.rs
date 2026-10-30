use std::collections::{HashMap, HashSet};

use warpui::{
    AppContext, Entity, EntityId, ModelContext, ModelHandle, SingletonEntity, WindowId,
};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent_conversations_model::AgentConversationEntryId;
use crate::ai::ambient_agents::AmbientAgentTaskId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConversationOrTaskId {
    ConversationId(AIConversationId),
    TaskId(AmbientAgentTaskId),
}

impl From<AgentConversationEntryId> for ConversationOrTaskId {
    fn from(id: AgentConversationEntryId) -> Self {
        match id {
            AgentConversationEntryId::Conversation(conversation_id) => {
                ConversationOrTaskId::ConversationId(conversation_id)
            }
            AgentConversationEntryId::AmbientRun(task_id) => ConversationOrTaskId::TaskId(task_id),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActiveAgentViewsEvent {}

pub struct ActiveAgentViewsModel;

impl ActiveAgentViewsModel {
    pub fn new() -> Self {
        Self
    }
}

impl Entity for ActiveAgentViewsModel {
    type Event = ActiveAgentViewsEvent;
}

impl SingletonEntity for ActiveAgentViewsModel {}
