use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warpui::{Entity, ModelContext, SingletonEntity};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::ambient_agents::AmbientAgentTaskId;

#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct AgentManagementFilters {
    pub owners: OwnerFilter,
    pub status: StatusFilter,
    pub source: SourceFilter,
    pub created_on: CreatedOnFilter,
    pub creator: CreatorFilter,
    pub artifact: ArtifactFilter,
    #[serde(default)]
    pub environment: EnvironmentFilter,
    #[serde(default)]
    pub harness: HarnessFilter,
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct OwnerFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct StatusFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct SourceFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct CreatedOnFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct CreatorFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentFilter;
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct HarnessFilter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AgentConversationEntryId {
    Conversation(AIConversationId),
    AmbientRun(AmbientAgentTaskId),
}

pub struct AgentConversationEntry;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentRunDisplayStatus {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentConversationNavigationSubject {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub enum AgentConversationsModelEvent {}

pub struct AgentConversationsModel;

impl AgentConversationsModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for AgentConversationsModel {
    type Event = AgentConversationsModelEvent;
}

impl SingletonEntity for AgentConversationsModel {}

pub trait AgentConversationListPolicy: Send {
    fn classify_entry(
        &self,
        _entry: &AgentConversationEntry,
        _app: &warpui::AppContext,
    ) -> AgentConversationListEntryState {
        AgentConversationListEntryState::Available
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentConversationListEntryState {
    Selected,
    OpenElsewhere,
    Available,
    Unavailable,
}
