use std::fmt::{Display, Formatter};
use std::str::FromStr;

use uuid::Uuid;
use warpui::{Entity, ModelContext, SingletonEntity, ViewContext};

use crate::ai::agent::conversation::AIConversation;
use crate::ai::agent::{FinishedAIAgentOutput, RenderableAIError};

#[derive(Clone, Debug)]
pub enum AmbientAgentEvent {
    TaskSpawned {
        task_id: AmbientAgentTaskId,
        run_id: String,
    },
    StateChanged {
        state: AmbientAgentTaskState,
        status_message: Option<TaskStatusMessage>,
    },
    SessionStarted {
        session_join_info: spawn::SessionJoinInfo,
    },
    TimedOut,
    AtCapacity,
}

pub struct AgentConfigSnapshot;
#[derive(Clone, Debug, Default)]
pub struct AmbientAgentTask;
#[derive(Clone, Debug, Default)]
pub struct AmbientAgentTaskState;
pub struct AmbientAgentLiveSessionState;
#[derive(Clone, Debug, Default)]
pub struct TaskStatusMessage;

pub mod github_auth_notifier;
pub mod scheduled;
pub mod spawn;
pub mod task;
pub mod telemetry;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AmbientAgentTaskId(Uuid);

impl AmbientAgentTaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for AmbientAgentTaskId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AmbientAgentTaskId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl TryFrom<String> for AmbientAgentTaskId {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Clone, Debug)]
pub enum AmbientConversationStatus {
    Success,
    Error {
        error: RenderableAIError,
    },
    Cancelled {
        reason: crate::ai::agent::CancellationReason,
    },
    Blocked {
        blocked_action: String,
    },
}

pub fn conversation_output_status_from_conversation(
    _conversation: &AIConversation,
) -> Option<AmbientConversationStatus> {
    None
}

pub fn cancel_task_with_toast<V: warpui::View>(
    task_id: AmbientAgentTaskId,
    ctx: &mut ViewContext<V>,
) {
    task::cancel_task_with_toast(task_id, ctx);
}

pub fn cancel_task_silently(task_id: AmbientAgentTaskId, ctx: &mut warpui::AppContext) {
    task::cancel_task_silently(task_id, ctx);
}

pub use github_auth_notifier::GitHubAuthNotifier;
pub use task::{
    AttachmentInput, HarnessAuthSecretsConfig, HarnessConfig, HarnessModelConfig, TaskAttachment,
    normalize_orchestrator_agent_name,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentSource {
    #[default]
    Cli,
    GitHubAction,
    CloudMode,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExecutionLocation {
    #[default]
    Unknown,
}
