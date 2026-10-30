pub mod conversation;
pub mod todos;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp_core::command::ExitCode;
use warpui::{Entity, ViewContext};

pub use ai::agent::action::{CreateDocumentsRequest, FileEdit};
pub use ai::agent::action_result::{
    AIAgentActionResultType, CreateDocumentsResult, EditDocumentsResult, RequestFileEditsResult,
};

pub struct AIAgentHarness;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CancellationReason {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EntrypointType {
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ImageContext {
    pub data: String,
    pub mime_type: String,
    pub file_name: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AIAgentActionId(Uuid);

#[derive(Clone, Debug, Default)]
pub struct AIAgentAttachment;

#[derive(Clone, Debug, Default)]
pub struct AIAgentOutput;

#[derive(Clone, Debug, Default)]
pub struct AIAgentTextSection;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIAgentActionType {
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
pub struct RenderableAIError;

#[derive(Clone, Debug)]
pub enum FinishedAIAgentOutput {
    Success,
    Error {
        error: RenderableAIError,
    },
    Cancelled {
        reason: CancellationReason,
    },
}

#[derive(Clone, Debug, Default)]
pub struct PassiveCodeDiffEntry {
    pub file_path: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIAgentOutputMessageType {
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct DiffSetHunk;

#[derive(Clone, Debug, Default)]
pub struct FileLocations;

#[derive(Clone, Debug, Default)]
pub struct AnyFileContent;

#[derive(Clone, Debug, Default)]
pub struct AgentReviewCommentBatch;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ServerOutputId(Uuid);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PassiveSuggestionTrigger {
    #[default]
    Unknown,
}

pub struct PassiveSuggestionResultType;
pub struct ShellCommandCompletedTrigger;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SuggestedLoggingId {
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct AIIdentifiers;

pub mod icons {
    pub fn yellow_running_icon() -> warp_core::ui::Icon {
        warp_core::ui::Icon::HelpCircle
    }

    pub fn yellow_stop_icon() -> warp_core::ui::Icon {
        warp_core::ui::Icon::HelpCircle
    }

    pub fn failed_icon() -> warp_core::ui::Icon {
        warp_core::ui::Icon::HelpCircle
    }

    pub fn in_progress_icon() -> warp_core::ui::Icon {
        warp_core::ui::Icon::HelpCircle
    }
}

pub struct ReadFilesRequest;
pub struct MessageId(uuid::Uuid);
pub struct AIAgentAction;
pub struct AIAgentOutputMessage;
pub struct AIAgentCitation;
pub struct AIAgentOutputStatus;
pub struct ProgrammingLanguage;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StaticQueryType {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIAgentPtyWriteMode {
    #[default]
    Unknown,
}

pub fn extract_user_query_mode(_input: &AIAgentInput) -> UserQueryMode {
    UserQueryMode::Agent
}

pub fn display_user_query_with_mode(_query: &str, _mode: UserQueryMode) -> String {
    _query.to_owned()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StartAgentExecutionMode {
    #[default]
    Unknown,
}

pub use conversation::{AIConversation, AIConversationId, ConversationStatus};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TodoOperation {
    #[default]
    Noop,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AIAgentExchangeId(Uuid);

#[derive(Clone, Debug, Default)]
pub struct AIAgentExchange;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UserQueryMode {
    #[default]
    Agent,
}

#[derive(Clone, Debug, Default)]
pub enum AIAgentInput {
    #[default]
    Empty,
    CodeReview {
        context: Arc<AIAgentContext>,
        review_comments: crate::code_review::comments::ReviewCommentBatch,
    },
}

#[derive(Clone, Debug, Default)]
pub struct AIAgentContext;

pub mod task {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct TaskId(uuid::Uuid);

    #[derive(Clone, Debug, Default)]
    pub struct Task;
}

pub mod action {
    #[derive(Clone, Debug, Default)]
    pub struct InsertReviewComment;
}

pub mod api {
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ServerConversationToken(String);

    impl ServerConversationToken {
        pub fn new(value: String) -> Self {
            Self(value)
        }
    }
}

pub mod redaction {
    pub fn redact_secrets(text: &str) -> String {
        text.to_owned()
    }
}

pub struct CurrentHead;
pub struct DiffBase;

#[derive(Clone, Debug)]
pub enum AskAIType {
    FromTextSelection {
        text: Arc<String>,
        populate_input_box: bool,
    },
    FromBlock {
        input: Arc<String>,
        output: Arc<String>,
        exit_code: ExitCode,
        block_index: crate::terminal::model::terminal_model::BlockIndex,
    },
    FromBlocks {
        block_indices: std::collections::HashSet<crate::terminal::model::terminal_model::BlockIndex>,
    },
    FromAICommandSearch {
        query: Arc<String>,
    },
}
