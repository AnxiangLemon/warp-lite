use std::fmt::Display;
use std::ops::Deref;

pub use input_classifier::InputType;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

/// A ID for an AI action generated as part of an `AIAgentOutput`.
///
/// The internal ID itself should be opaque to all callers. This ID may be relayed back to the AI with
/// the `AIAgentActionResult` from the action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AIAgentActionId(String);

impl From<String> for AIAgentActionId {
    fn from(value: String) -> Self {
        AIAgentActionId(value)
    }
}

impl From<AIAgentActionId> for String {
    fn from(value: AIAgentActionId) -> Self {
        value.0
    }
}

impl Display for AIAgentActionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<crate::persistence::model::AIAgentActionId> for AIAgentActionId {
    fn from(value: crate::persistence::model::AIAgentActionId) -> Self {
        Self(value.0)
    }
}

impl From<AIAgentActionId> for crate::persistence::model::AIAgentActionId {
    fn from(value: AIAgentActionId) -> Self {
        crate::persistence::model::AIAgentActionId(value.0)
    }
}

/// A globally unique ID for an `AIAgentExchange`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AIAgentExchangeId(Uuid);

impl Display for AIAgentExchangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AIAgentExchangeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AIAgentExchangeId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<String> for AIAgentExchangeId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::try_parse(&value)?))
    }
}

/// A globally unique ID for a conversation with an AI agent.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AIConversationId(Uuid);

impl Display for AIConversationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AIConversationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AIConversationId {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<String> for AIConversationId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::try_parse(&value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(String);

impl TaskId {
    pub fn new(id: String) -> Self {
        TaskId(id)
    }
}

impl From<TaskId> for String {
    fn from(id: TaskId) -> Self {
        id.0
    }
}

impl Deref for TaskId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Configuration for the terminal pane's input.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputConfig {
    /// The type of the terminal input.
    pub input_type: InputType,

    /// If `true`, we will not attempt to auto-detect the best input type.
    pub is_locked: bool,
}

impl InputConfig {
    pub fn with_toggled_type(self) -> Self {
        let input_type = if self.input_type.is_ai() {
            InputType::Shell
        } else {
            InputType::AI
        };
        Self { input_type, ..self }
    }

    pub fn with_shell_type(self) -> Self {
        Self {
            input_type: InputType::Shell,
            ..self
        }
    }

    pub fn with_input_type(self, input_type: InputType) -> Self {
        Self { input_type, ..self }
    }

    pub fn locked(self) -> Self {
        Self {
            is_locked: true,
            ..self
        }
    }

    pub fn is_ai(&self) -> bool {
        self.input_type == InputType::AI
    }

    pub fn is_shell(&self) -> bool {
        self.input_type == InputType::Shell
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum UserTakeOverReason {
    Manual,
    /// The user interrupted the command and took control. `should_auto_resume` is `true` for a
    /// live interrupt (e.g. Ctrl-C) that keeps the conversation alive so it resumes once the
    /// command completes, and `false` for teardown flows (stop/rewind) that have cancelled it.
    Stop {
        should_auto_resume: bool,
    },
    /// The agent explicitly transferred control to the user via the
    /// TransferShellCommandControlToUser tool call.
    TransferFromAgent {
        /// The reason the agent gave for transferring control.
        reason: String,
    },
}

impl<'de> Deserialize<'de> for UserTakeOverReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // `Current` mirrors the derived shape so serde can parse it without recursing back into
        // this impl. `LegacyStop` accepts the bare `"Stop"` persisted before `should_auto_resume`
        // existed: an externally tagged enum can't accept `Stop` as both a unit (legacy) and a
        // struct (current) variant, and `#[serde(default)]` can't bridge the two, so the forms are
        // unioned as `untagged`.
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Current(Current),
            LegacyStop(LegacyStop),
        }

        #[derive(Deserialize)]
        enum Current {
            Manual,
            Stop { should_auto_resume: bool },
            TransferFromAgent { reason: String },
        }

        #[derive(Deserialize)]
        enum LegacyStop {
            Stop,
        }

        Ok(match Wire::deserialize(deserializer)? {
            Wire::Current(Current::Manual) => Self::Manual,
            Wire::Current(Current::Stop { should_auto_resume }) => {
                Self::Stop { should_auto_resume }
            }
            Wire::Current(Current::TransferFromAgent { reason }) => {
                Self::TransferFromAgent { reason }
            }
            Wire::LegacyStop(LegacyStop::Stop) => Self::Stop {
                should_auto_resume: false,
            },
        })
    }
}

impl UserTakeOverReason {
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop { .. })
    }

    pub fn is_transfer_from_agent(&self) -> bool {
        matches!(self, Self::TransferFromAgent { .. })
    }

    /// Returns `true` if the conversation should resume once the user-controlled command
    /// completes. Only a teardown `Stop` opts out.
    pub fn should_auto_resume(&self) -> bool {
        match self {
            Self::Manual | Self::TransferFromAgent { .. } => true,
            Self::Stop { should_auto_resume } => *should_auto_resume,
        }
    }

    pub fn transfer_reason(&self) -> Option<&str> {
        match self {
            Self::TransferFromAgent { reason } => Some(reason.as_str()),
            _ => None,
        }
    }
}

/// Represents which party is in control of the active long running command.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LongRunningCommandControlState {
    /// The agent is in control.
    ///
    /// When the agent has control, the user cannot submit input to the command.
    Agent {
        /// `true` if the agent is blocked on approval from the user for submitting input.
        is_blocked: bool,
        /// `true` if agent responses should be hidden in the UI.
        should_hide_responses: bool,
    },
    /// The user is in control.
    User { reason: UserTakeOverReason },
}

impl LongRunningCommandControlState {
    pub fn is_agent_in_control(&self) -> bool {
        matches!(self, Self::Agent { .. })
    }

    pub fn is_agent_blocked(&self) -> bool {
        matches!(
            self,
            Self::Agent {
                is_blocked: true,
                ..
            }
        )
    }

    pub fn is_user_in_control(&self) -> bool {
        matches!(self, Self::User { .. })
    }

    /// Returns `true` if a completing user-controlled command should auto-resume the conversation.
    pub fn should_auto_resume(&self) -> bool {
        match self {
            Self::Agent { .. } => false,
            Self::User { reason } => reason.should_auto_resume(),
        }
    }

    pub fn should_hide_responses(&self) -> bool {
        matches!(
            self,
            Self::Agent {
                should_hide_responses: true,
                ..
            }
        )
    }

    pub fn user_take_over_reason(&self) -> Option<&UserTakeOverReason> {
        match &self {
            LongRunningCommandControlState::Agent { .. } => None,
            LongRunningCommandControlState::User { reason } => Some(reason),
        }
    }
}

/// Blocklist AI metadata associated with this block.
#[derive(Debug, Clone)]
pub struct AgentInteractionMetadata {
    /// The ID of the `AIAgentAction` associated with this block's requested command execution.
    /// This is optional because not all AI-related blocks are associated with a requested command.
    pub(super) requested_command_action_id: Option<AIAgentActionId>,

    /// The ID of the conversation to which this action belongs.
    pub(super) conversation_id: AIConversationId,

    /// The task ID for the CLI subagent interaction with this block if any.
    pub(super) subagent_task_id: Option<TaskId>,

    /// State governing user/agent interaction with the command in this block.
    pub(super) long_running_control_state: Option<LongRunningCommandControlState>,

    /// `true` if the agent has previously written to this block.
    pub(super) has_agent_written_to_block: bool,

    /// `true` if this block should be hidden from the user (as is the case with AI-requested
    /// commands, for example).
    pub(super) should_hide_block: bool,
}

impl AgentInteractionMetadata {
    /// Creates a new metadata instance with fully specified fields.
    pub fn new(
        requested_command_action_id: Option<AIAgentActionId>,
        conversation_id: AIConversationId,
        subagent_task_id: Option<TaskId>,
        long_running_control_state: Option<LongRunningCommandControlState>,
        has_agent_written_to_block: bool,
        should_hide_block: bool,
    ) -> Self {
        AgentInteractionMetadata {
            requested_command_action_id,
            conversation_id,
            subagent_task_id,
            long_running_control_state,
            has_agent_written_to_block,
            should_hide_block,
        }
    }

    /// Convenience constructor for the common "hidden by default" case used for requested commands.
    pub fn new_hidden(
        requested_command_action_id: AIAgentActionId,
        conversation_id: AIConversationId,
    ) -> Self {
        Self::new(
            Some(requested_command_action_id),
            conversation_id,
            None,
            None,
            false,
            true,
        )
    }

    pub fn requested_command_action_id(&self) -> Option<&AIAgentActionId> {
        self.requested_command_action_id.as_ref()
    }

    pub fn conversation_id(&self) -> &AIConversationId {
        &self.conversation_id
    }

    pub fn subagent_task_id(&self) -> Option<&TaskId> {
        self.subagent_task_id.as_ref()
    }

    pub fn is_agent_in_control(&self) -> bool {
        self.long_running_control_state
            .as_ref()
            .is_some_and(|state| state.is_agent_in_control())
    }

    pub fn long_running_control_state(&self) -> Option<&LongRunningCommandControlState> {
        self.long_running_control_state.as_ref()
    }

    pub fn has_agent_written_to_block(&self) -> bool {
        self.has_agent_written_to_block
    }

    pub fn should_hide_block(&self) -> bool {
        self.should_hide_block
    }
}

#[cfg(test)]
#[path = "session_types_tests.rs"]
mod tests;
