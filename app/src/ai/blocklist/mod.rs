//! This module contains model, controller, and view logic for Blocklist AI.
mod action_model;
pub mod agent_view;
pub mod block;
mod child_agent_launch;
pub mod code_block;
mod context_model;
mod controller;
pub(crate) mod conversation_selection;
pub(crate) mod diff_storage;
pub(crate) mod diff_types;
pub(crate) mod handoff;

pub(crate) mod local_agent_task_sync_model;
pub(crate) mod orchestration_event_streamer;
pub(crate) mod orchestration_events;
pub(crate) mod orchestration_topology;
mod passive_suggestions;
pub(crate) mod queued_query;
pub(super) use controller::RequestInput;
pub mod history_model;
pub mod inline_action;
mod input_mode_policy;
mod input_model;
mod permissions;
mod persistence;
pub mod prompt;
pub mod suggested_agent_mode_workflow_modal;
pub mod suggested_rule_modal;
mod suggestion_chip_view;
pub mod summarization_cancel_dialog;
pub(crate) mod telemetry;
pub mod usage;

pub(crate) mod codebase_index_speedbump_banner;
pub(crate) mod telemetry_banner;
pub(crate) mod view_util;

// Consumed by `tui_export` for the `warp_tui` frontend.
#[cfg_attr(not(feature = "tui"), allow(unused_imports))]
pub use action_model::AIActionStatus;
// Consumed by `tui_export` for the `warp_tui` frontend.
#[cfg(feature = "agent_runtime")]
pub use action_model::ShellCommandExecutorEvent;
#[cfg(feature = "agent_runtime")]
pub(crate) use action_model::recording_controller::RecordingController;
#[cfg(not(target_family = "wasm"))]
#[cfg(feature = "agent_runtime")]
pub(crate) use action_model::recording_finalize::{
    FinalizeReason, finalize_recording_for_conversation,
};
#[cfg(feature = "tui")]
pub use action_model::{
    AskUserQuestionExecutor, NewConversationDecision, RequestFileEditsExecutor,
};
pub use action_model::{BlocklistAIActionEvent, BlocklistAIActionModel, ShellCommandExecutor};
#[cfg_attr(target_family = "wasm", allow(unused_imports))]
pub(crate) use action_model::{
    FileReadResult, ReadFileContextResult, RequestFileEditsFormatKind, apply_edits,
    read_local_file_context,
};
// Consumed by `tui_export` for the `warp_tui` frontend.
#[cfg(feature = "tui")]
pub use action_model::{RunAgentsExecutor, RunAgentsExecutorEvent, RunAgentsSpawningSnapshot};
// Consumed by `tui_export` for the `warp_tui` frontend's child-agent
// materializer, in addition to the GUI pane-group dispatch.
#[cfg_attr(
    any(target_family = "wasm", not(feature = "tui")),
    allow(unused_imports)
)]
pub use action_model::{
    StartAgentExecutor, StartAgentExecutorEvent, StartAgentOutcome, StartAgentRequest,
    StartAgentRequestId,
};
#[cfg(feature = "agent_runtime")]
pub(crate) use block::AIBlockEvent;
#[cfg(feature = "agent_runtime")]
pub(crate) use block::init;
#[cfg(any(test, feature = "integration_tests"))]
#[cfg(feature = "agent_runtime")]
pub(crate) use block::model::testing::FakeAIBlockModel;
pub(crate) use block::{AIBlock, RequestedEditResolution, model};
pub use block::{keyboard_navigable_buttons, toggleable_items};
#[cfg(feature = "agent_runtime")]
pub use child_agent_launch::inherit_child_agent_settings;
#[cfg(not(target_family = "wasm"))]
#[cfg_attr(not(feature = "tui"), allow(unused_imports))]
pub use child_agent_launch::{
    PreparedLocalOzChildLaunch, apply_child_agent_model_override, prepare_local_oz_child_launch,
};
#[cfg(feature = "tui")]
pub use context_model::PendingAttachmentSummary;
#[cfg(not(feature = "tui"))]
#[cfg(feature = "agent_runtime")]
pub(crate) use context_model::block_context_from_terminal_model;
#[cfg(feature = "tui")]
#[cfg(feature = "agent_runtime")]
pub use context_model::block_context_from_terminal_model;
pub use context_model::{
    AttachmentType, BlocklistAIContextEvent, BlocklistAIContextModel, PendingAttachment,
    PendingFile,
};
pub use controller::BlocklistAIController;
#[cfg(feature = "agent_runtime")]
pub(crate) use controller::SlashCommandRequest;
pub use controller::input_context::{
    BLOCK_CONTEXT_ATTACHMENT_REGEX, DIFF_HUNK_ATTACHMENT_REGEX, DRIVE_OBJECT_ATTACHMENT_REGEX,
};
#[cfg(test)]
#[cfg(feature = "agent_runtime")]
pub(crate) use controller::response_stream::ResponseStream;
pub(crate) use controller::response_stream::ResponseStreamId;
pub(crate) use controller::{BlocklistAIControllerEvent, ClientIdentifiers, SessionContext};
#[cfg(feature = "agent_runtime")]
pub(crate) use conversation_selection::{ConversationSelection, PendingQueryState};
pub(crate) use conversation_selection::{ConversationSelectionEvent, ConversationSelectionHandle};
pub(crate) use history_model::{
    AIQueryHistory, AIQueryHistoryOutputStatus, BeginConversationRenameError,
    BlocklistAIHistoryEvent, BlocklistAIHistoryModel, ConversationStatusUpdate,
};
#[cfg(feature = "agent_runtime")]
pub(crate) use history_model::{FORK_PREFIX, PRE_REWIND_PREFIX};
// The policy types are re-exported for the TUI frontend via `tui_export`.
#[cfg_attr(not(feature = "tui"), allow(unused_imports))]
pub use input_mode_policy::{InputModePolicy, InputModePolicyHandle, PolicyConfigUpdate};
pub(crate) use input_model::BlocklistAIInputEvent;
pub use input_model::{
    BlocklistAIInputModel, InputConfig, InputType, InputTypeAutoDetectionSource,
};
#[cfg(feature = "agent_runtime")]
pub(crate) use passive_suggestions::{
    LegacyPassiveSuggestionsEvent, LegacyPassiveSuggestionsModel, MaaPassiveSuggestionsEvent,
    MaaPassiveSuggestionsModel, PassiveSuggestionsModels,
};
pub use permissions::{BlocklistAIPermissions, CommandExecutionPermissionAllowedReason};
#[cfg_attr(target_family = "wasm", allow(unused))]
#[cfg(any(test, feature = "agent_runtime"))]
pub(crate) use persistence::PersistedAIInputType;
#[cfg_attr(target_family = "wasm", allow(unused))]
#[cfg(feature = "agent_runtime")]
pub use persistence::maybe_build_ai_query_upsert_event;
pub(crate) use persistence::{PersistedAIInput, SerializedBlockListItem};
#[cfg(feature = "agent_runtime")]
pub(crate) use queued_query::{AutofireAction, QueuedQuery, is_lrc_auto_queue_active};
#[cfg_attr(not(feature = "tui"), allow(unused_imports))]
pub use queued_query::{QueuedQueryEvent, QueuedQueryModel};
pub(crate) use queued_query::{QueuedQueryId, QueuedQueryOrigin};
pub use suggestion_chip_view::*;
#[cfg(feature = "agent_runtime")]
pub(crate) use view_util::NEW_AGENT_PANE_LABEL;
pub use view_util::error_color;
#[cfg(feature = "agent_runtime")]
pub(crate) use view_util::render_ai_follow_up_icon;
pub(crate) use view_util::{
    ATTACH_AS_AGENT_MODE_CONTEXT_TEXT, CLAUDE_ORANGE, ai_brand_color, ai_indicator_height,
    format_credits, get_ai_block_overflow_menu_element_position_id,
    get_attached_blocks_chip_element_position_id, render_ai_agent_mode_icon,
};

pub use crate::ai::blocklist::block::{AIBlockResponseRating, TextLocation, secret_redaction};
