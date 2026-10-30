//! Minimal blocklist AI stubs for warp_lite.

pub mod agent_view;
pub mod handoff;
pub mod persistence;
pub mod prompt;

mod recording;
pub use recording::RecordingController;

use lazy_static::lazy_static;
use regex::Regex;
use uuid::Uuid;
use warpui::elements::Empty;
use warpui::{
    AppContext, Entity, EntityId, ModelContext, ModelHandle, SingletonEntity, View, ViewContext,
};

use crate::ai::agent::conversation::{AIConversation, AIConversationId, ConversationStatus};
use crate::ai::agent::{AIAgentExchange, AIAgentExchangeId, AIAgentInput, task::Task};
use crate::terminal::model::TerminalModel;

pub fn init(_app: &mut AppContext) {}

pub mod keyboard_navigable_buttons;
pub mod toggleable_items;
pub mod code_block;

pub mod suggested_rule_modal {
    use warpui::elements::{Element, Empty, ParentElement};
    use warpui::{AppContext, Entity, View, ViewContext};

    pub fn init(_app: &mut AppContext) {}

    #[derive(Clone, Debug)]
    pub struct SuggestedRuleAndId;

    #[derive(Clone, Debug)]
    pub enum SuggestedRuleModalEvent {
        AddNewRule { rule: SuggestedRuleAndId },
        OpenRuleForEditing { rule: SuggestedRuleAndId },
        Close,
    }

    pub struct SuggestedRuleModal;

    impl SuggestedRuleModal {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for SuggestedRuleModal {
        type Event = SuggestedRuleModalEvent;
    }

    impl View for SuggestedRuleModal {
        fn ui_name() -> &'static str {
            "SuggestedRuleModalLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub mod summarization_cancel_dialog {
    use warpui::elements::{Element, Empty, ParentElement};
    use warpui::{AppContext, Entity, View, ViewContext, ViewHandle};

    pub struct SummarizationCancelDialog;

    impl Entity for SummarizationCancelDialog {
        type Event = ();
    }

    impl View for SummarizationCancelDialog {
        fn ui_name() -> &'static str {
            "SummarizationCancelDialogLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueuedQueryId(Uuid);

#[derive(Clone, Debug, Default)]
pub struct QueuedQuery;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QueuedQueryOrigin {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AutofireAction {
    #[default]
    None,
}

pub fn is_lrc_auto_queue_active(_app: &AppContext) -> bool {
    false
}

pub fn ai_brand_color(_app: &AppContext) -> pathfinder_color::ColorU {
    pathfinder_color::ColorU::new(0, 0, 0, 255)
}

pub const ATTACH_AS_AGENT_MODE_CONTEXT_TEXT: &str = "";
pub const PRE_REWIND_PREFIX: &str = "";

pub fn get_ai_block_overflow_menu_element_position_id(_block_id: EntityId) -> String {
    String::new()
}

pub fn get_attached_blocks_chip_element_position_id(_terminal_view_id: EntityId) -> String {
    String::new()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Shell,
    AI,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputTypeAutoDetectionSource {
    #[default]
    ManualToggle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputConfig {
    pub input_type: InputType,
}

impl InputConfig {
    pub fn shell() -> Self {
        Self {
            input_type: InputType::Shell,
        }
    }
}

#[derive(Clone, Debug)]
pub enum BlocklistAIInputEvent {}

pub struct BlocklistAIInputModel;

impl BlocklistAIInputModel {
    pub fn new(
        _initial_config: InputConfig,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    pub fn set_input_config(&mut self, _config: InputConfig, _ctx: &mut ModelContext<Self>) {}
}

impl Entity for BlocklistAIInputModel {
    type Event = BlocklistAIInputEvent;
}

#[derive(Clone, Debug)]
pub enum PendingAttachment {
    Image(crate::ai::agent::ImageContext),
    File(PendingFile),
}

#[derive(Clone, Debug)]
pub enum BlocklistAIContextEvent {}

pub struct BlocklistAIContextModel;

impl BlocklistAIContextModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn pending_context_block_ids(&self) -> Vec<EntityId> {
        vec![]
    }

    pub fn pending_context(
        &self,
        _ctx: &AppContext,
        _is_user_query: bool,
        _cwd: Option<&str>,
    ) -> crate::ai::agent::AIAgentContext {
        crate::ai::agent::AIAgentContext
    }

    pub fn register_diff_hunk_attachment(
        &mut self,
        _diff_hunk_id: String,
        _attachment: crate::ai::agent::AIAgentAttachment,
    ) {
    }
}

impl Entity for BlocklistAIContextModel {
    type Event = BlocklistAIContextEvent;
}

#[derive(Clone, Debug)]
pub enum BlocklistAIActionEvent {}

pub struct BlocklistAIActionModel;

impl BlocklistAIActionModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for BlocklistAIActionModel {
    type Event = BlocklistAIActionEvent;
}

#[derive(Clone, Debug)]
pub enum BlocklistAIControllerEvent {}

pub struct BlocklistAIController;

impl BlocklistAIController {
    pub fn new(
        _input: ModelHandle<BlocklistAIInputModel>,
        _context: ModelHandle<BlocklistAIContextModel>,
        _action: ModelHandle<BlocklistAIActionModel>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }
}

impl Entity for BlocklistAIController {
    type Event = BlocklistAIControllerEvent;
}

#[derive(Clone, Debug)]
pub enum BlocklistAIHistoryEvent {}

pub struct BlocklistAIHistoryModel;

impl BlocklistAIHistoryModel {
    pub fn new(
        _ai_queries: Vec<()>,
        _nld_prompts: Vec<String>,
        _conversations: &[()],
    ) -> Self {
        Self
    }

    pub fn conversation(
        &self,
        _conversation_id: &AIConversationId,
    ) -> Option<&AIConversation> {
        None
    }

    pub fn get_server_conversation_metadata(
        &self,
        _conversation_id: &AIConversationId,
    ) -> Option<&crate::ai::agent::conversation::ServerAIConversationMetadata> {
        None
    }

    pub fn latest_exchange_across_all_conversations(
        &self,
        _terminal_view_id: EntityId,
    ) -> Option<&AIAgentExchange> {
        None
    }
}

impl Entity for BlocklistAIHistoryModel {
    type Event = BlocklistAIHistoryEvent;
}

impl SingletonEntity for BlocklistAIHistoryModel {}

pub struct BlocklistAIPermissions;

impl BlocklistAIPermissions {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for BlocklistAIPermissions {
    type Event = ();
}

impl SingletonEntity for BlocklistAIPermissions {}

pub struct QueuedQueryModel;

impl QueuedQueryModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for QueuedQueryModel {
    type Event = ();
}

impl SingletonEntity for QueuedQueryModel {}

pub struct PassiveSuggestionsModels;

impl PassiveSuggestionsModels {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

#[derive(Clone, Debug)]
pub enum LegacyPassiveSuggestionsEvent {}

pub struct LegacyPassiveSuggestionsModel;

#[derive(Clone, Debug)]
pub enum MaaPassiveSuggestionsEvent {}

pub struct MaaPassiveSuggestionsModel;

pub mod block {
        use warpui::elements::{Empty, ParentElement};
        use warpui::{AppContext, Entity, View};

        pub mod cli_controller {
        use warpui::{Entity, ModelContext};

        #[derive(Clone, Debug)]
        pub enum CLISubagentEvent {}

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum LongRunningCommandControlState {
            #[default]
            Unknown,
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum UserTakeOverReason {
            #[default]
            Unknown,
        }

        pub struct CLISubagentController;

        impl CLISubagentController {
            pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
                Self
            }
        }

        impl Entity for CLISubagentController {
            type Event = CLISubagentEvent;
        }
    }

    pub mod cli {
        use warpui::elements::{Element, Empty, ParentElement};
        use warpui::{AppContext, Entity, View, ViewContext};

        #[derive(Clone, Debug)]
        pub enum CLISubagentViewEvent {}

        pub struct CLISubagentView;

        impl Entity for CLISubagentView {
            type Event = CLISubagentViewEvent;
        }

        impl View for CLISubagentView {
            fn ui_name() -> &'static str {
                "CLISubagentViewLite"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn Element> {
                Empty::new().finish()
            }
        }
    }

    pub mod view_impl {
        pub struct AIBlockViewImpl;
        pub const CONTENT_HORIZONTAL_PADDING: f32 = 0.0;
        pub const CONTENT_ITEM_VERTICAL_MARGIN: f32 = 0.0;
        pub struct WithContentItemSpacing;

        pub mod output {
            pub struct LinkActionConstructors<A>(std::marker::PhantomData<A>);
        }
    }

    pub struct TextLocation;

    pub struct PendingUserQueryBlock;
    pub struct PendingExchangeBlock;

    #[derive(Clone, Debug)]
    pub enum PendingUserQueryBlockEvent {
        Dismissed,
        SendNow,
        TextSelected,
    }

    pub mod secret_redaction {
        use warpui::elements::SecretRange;

        use crate::terminal::model::secrets::{SecretLevel, SecretsRegex};

        pub fn redact_secrets(text: &str) -> String {
            text.to_owned()
        }

        pub fn find_secrets_in_text_with_levels_using_regex(
            _text: &str,
            _secrets_regex: &SecretsRegex,
        ) -> Vec<(SecretRange, SecretLevel)> {
            vec![]
        }
    }

    pub use super::{keyboard_navigable_buttons, toggleable_items};

    pub mod status_bar {
        use warpui::elements::{Element, Empty, ParentElement};
        use warpui::{AppContext, Entity, ModelHandle, View, ViewContext};

        #[derive(Clone, Debug)]
        pub enum BlocklistAIStatusBarEvent {}

        pub fn init(_app: &mut AppContext) {}

        pub struct BlocklistAIStatusBar;

        impl BlocklistAIStatusBar {
            pub fn new(
                _controller: ModelHandle<super::super::BlocklistAIController>,
                _ctx: &mut ViewContext<Self>,
            ) -> Self {
                Self
            }

            pub fn set_ambient_agent_view_model(
                &mut self,
                _model: ModelHandle<crate::terminal::view::ambient_agent::AmbientAgentViewModel>,
                _ctx: &mut ViewContext<Self>,
            ) {
            }
        }

        impl Entity for BlocklistAIStatusBar {
            type Event = BlocklistAIStatusBarEvent;
        }

        impl View for BlocklistAIStatusBar {
            fn ui_name() -> &'static str {
                "BlocklistAIStatusBarLite"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn Element> {
                Empty::new().finish()
            }
        }
    }

    pub mod number_shortcut_buttons {
        use warpui::AppContext;

        pub fn init(_app: &mut AppContext) {}
    }

    #[derive(Clone, Debug)]
    pub enum AIBlockEvent {}

    pub struct AIBlock;

    impl Entity for AIBlock {
        type Event = AIBlockEvent;
    }

    impl View for AIBlock {
        fn ui_name() -> &'static str {
            "AIBlockLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn warpui::elements::Element> {
            Empty::new().finish()
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum FinishReason {
        #[default]
        Unknown,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AIBlockAction {
        Unknown,
    }
}

pub mod conversation_selection;

pub mod orchestration_events {
    use warpui::{Entity, ModelContext, SingletonEntity};

    pub struct OrchestrationEventService;

    impl OrchestrationEventService {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for OrchestrationEventService {
        type Event = ();
    }

    impl SingletonEntity for OrchestrationEventService {}
}

pub mod local_agent_task_sync_model {
    use warpui::{Entity, ModelContext, SingletonEntity};

    pub struct LocalAgentTaskSyncModel;

    impl LocalAgentTaskSyncModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for LocalAgentTaskSyncModel {
        type Event = ();
    }

    impl SingletonEntity for LocalAgentTaskSyncModel {}
}

pub mod orchestration_event_streamer {
    use warpui::{Entity, ModelContext, SingletonEntity};

    #[derive(Clone, Debug)]
    pub enum OrchestrationEventStreamerEvent {}

    pub struct OrchestrationEventStreamer;

    impl OrchestrationEventStreamer {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for OrchestrationEventStreamer {
        type Event = OrchestrationEventStreamerEvent;
    }

    impl SingletonEntity for OrchestrationEventStreamer {}
}

pub mod codebase_index_speedbump_banner {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum CodebaseIndexSpeedbumpBannerAction {
        #[default]
        Unknown,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum CodebaseIndexSpeedbumpBannerState {
        #[default]
        Hidden,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum VisibilityState {
        #[default]
        Hidden,
    }
}

pub mod telemetry_banner {
    use warpui::AppContext;

    pub struct TelemetryBanner;

    pub fn should_collect_ai_ugc_telemetry(_app: &AppContext) -> bool {
        false
    }
}

pub mod diff_storage {
    pub type SaveFuture = futures::future::Ready<()>;
    pub struct DiffStorageHelper;
}

pub mod diff_types {
    use std::ops::Range;

    use ai::diff_validation::DiffType;
    use warp_core::HostId;

    #[derive(Clone)]
    #[cfg_attr(debug_assertions, derive(Debug))]
    pub struct DiffBase {
        pub content: String,
        pub file_path: String,
    }

    #[derive(Clone)]
    #[cfg_attr(debug_assertions, derive(Debug))]
    pub struct FileDiff {
        pub base: DiffBase,
        pub diff_type: DiffType,
    }

    #[derive(Clone, Debug)]
    pub enum DiffSessionType {
        Local,
        Remote(HostId),
    }

    pub fn changed_lines_from_op(_diff_type: &DiffType) -> Vec<Range<usize>> {
        vec![]
    }
}

pub mod inline_action {
    pub mod inline_action_header {
        pub struct ExpandedConfig;
        pub struct HeaderConfig;
        pub const INLINE_ACTION_HORIZONTAL_PADDING: f32 = 0.0;
        pub const INLINE_ACTION_HEADER_VERTICAL_PADDING: f32 = 0.0;

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum InteractionMode {
            #[default]
            Unknown,
        }

        pub fn render_header() -> Option<Box<dyn warpui::elements::Element>> {
            None
        }
    }

    pub mod inline_action_icons {
        pub fn icon_size() -> f32 {
            0.0
        }

        pub fn green_check_icon() -> warp_core::ui::Icon {
            warp_core::ui::Icon::HelpCircle
        }

        pub fn cancelled_icon() -> warp_core::ui::Icon {
            warp_core::ui::Icon::HelpCircle
        }

        pub fn red_x_icon(_appearance: &crate::appearance::Appearance) -> warp_core::ui::Icon {
            warp_core::ui::Icon::HelpCircle
        }
    }

    pub mod requested_command {
        pub struct RenderableAction;
        pub const VIEWING_COMMAND_DETAIL_MESSAGE: &str = "";
    }
    pub mod requested_action {
        pub struct RenderableAction;
    }

    pub mod orchestration_controls {
        pub const ORCHESTRATION_WARP_WORKER_HOST: &str = "";
    }

    pub mod code_diff_view {
        use warpui::elements::{Element, Empty, ParentElement};
        use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

        use crate::code::buffer_location::LocalOrRemotePath;
        use crate::pane_group::focus_state::PaneFocusHandle;
        use crate::pane_group::pane::{BackingView, view};
        use crate::pane_group::PaneEvent;

        #[derive(Clone, Debug)]
        pub enum CodeDiffViewEvent {
            Pane(PaneEvent),
        }

        #[derive(Clone, Debug)]
        pub enum CodeDiffViewAction {}

        pub struct CodeDiffView {
            focus_handle: Option<PaneFocusHandle>,
        }

        impl CodeDiffView {
            pub fn primary_file_location(&self, _ctx: &AppContext) -> Option<LocalOrRemotePath> {
                None
            }
        }

        impl Entity for CodeDiffView {
            type Event = CodeDiffViewEvent;
        }

        impl View for CodeDiffView {
            fn ui_name() -> &'static str {
                "CodeDiffViewLite"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn Element> {
                Empty::new().finish()
            }
        }

        impl TypedActionView for CodeDiffView {
            type Action = CodeDiffViewAction;

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl BackingView for CodeDiffView {
            type PaneHeaderOverflowMenuAction = CodeDiffViewAction;
            type CustomAction = ();
            type AssociatedData = ();

            fn handle_pane_header_overflow_menu_action(
                &mut self,
                _action: &Self::PaneHeaderOverflowMenuAction,
                _ctx: &mut ViewContext<Self>,
            ) {
            }

            fn close(&mut self, ctx: &mut ViewContext<Self>) {
                ctx.emit(CodeDiffViewEvent::Pane(PaneEvent::Close));
            }

            fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
                ctx.focus_self();
            }

            fn render_header_content(
                &self,
                _ctx: &view::HeaderRenderContext<'_>,
                _app: &AppContext,
            ) -> view::HeaderContent {
                view::HeaderContent::Standard(view::StandardHeader {
                    title: "Requested Edit".to_string(),
                    title_secondary: None,
                    title_style: None,
                    title_clip_config: warpui::text_layout::ClipConfig::start(),
                    title_max_width: None,
                    left_of_title: None,
                    right_of_title: None,
                    left_of_overflow: None,
                    options: view::StandardHeaderOptions {
                        hide_close_button: true,
                        ..Default::default()
                    },
                })
            }

            fn set_focus_handle(
                &mut self,
                focus_handle: PaneFocusHandle,
                _ctx: &mut ViewContext<Self>,
            ) {
                self.focus_handle = Some(focus_handle);
            }
        }
    }
}

pub mod history_model {
    pub use super::{
        BlocklistAIHistoryEvent, BlocklistAIHistoryModel, BlocklistAIPermissions,
    };

    pub struct CloudConversationData;
    pub struct AIQueryHistory;
    pub struct CLIAgentConversation;

    pub fn load_conversation_from_server() {}
}

pub mod model;
pub use model::FakeAIBlockModel;
pub use model::{
    AIBlockModel, AIBlockModelHelper, AIBlockModelImpl, AIBlockOutputStatus, AIRequestType,
    OutputStatusUpdateCallback, PassiveRequestType,
};

pub mod orchestration_topology {
    use warpui::AppContext;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum OrchestrationNavigationDirection {
        #[default]
        Next,
    }

    pub fn orchestration_aware_conversation_status() -> Option<()> {
        None
    }

    pub fn has_local_orchestrated_children(_app: &AppContext) -> bool {
        false
    }
}

pub mod suggested_agent_mode_workflow_modal {
    use warpui::elements::{Element, Empty, ParentElement};
    use warpui::{AppContext, Entity, View, ViewContext};

    pub struct SuggestedAgentModeWorkflowAndId;

    #[derive(Clone, Debug)]
    pub enum SuggestedAgentModeWorkflowModalEvent {
        Close,
    }

    pub struct SuggestedAgentModeWorkflowModal;

    impl SuggestedAgentModeWorkflowModal {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for SuggestedAgentModeWorkflowModal {
        type Event = SuggestedAgentModeWorkflowModalEvent;
    }

    impl View for SuggestedAgentModeWorkflowModal {
        fn ui_name() -> &'static str {
            "SuggestedAgentModeWorkflowModalLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub mod usage {
    pub mod conversation_usage_view {
        pub struct ConversationUsageInfo;
        pub struct ConversationUsageView;
        pub struct TimingInfo;
    }
}

pub mod action_model {
    use warpui::{Entity, ModelContext};

    #[derive(Clone, Debug)]
    pub enum ShellCommandExecutorEvent {}

    pub struct ShellCommandExecutor;

    #[derive(Clone, Debug)]
    pub enum StartAgentExecutorEvent {}

    pub struct StartAgentExecutor;

    #[derive(Clone, Debug, Default)]
    pub struct StartAgentRequest;
}

pub fn block_context_from_terminal_model(
    _model: &TerminalModel,
    _app: &AppContext,
) -> crate::ai::block_context::BlockContext {
    crate::ai::block_context::BlockContext
}

pub(crate) fn should_show_task_in_blocklist(_task: &Task) -> bool {
    false
}

pub(crate) fn conversation_would_render_in_blocklist(_conversation: &AIConversation) -> bool {
    false
}

pub(crate) fn exchanges_for_blocklist(_conversation: &AIConversation) -> Vec<&AIAgentExchange> {
    vec![]
}

pub use persistence::{
    maybe_build_ai_query_upsert_event, AIQueryHistoryOutputStatus, PersistedAIInput,
    PersistedAIInputType, SerializedBlockListItem,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentType {
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct PendingFile {
    pub file_name: String,
    pub file_path: std::path::PathBuf,
    pub mime_type: String,
}

#[derive(Clone, Debug)]
pub enum QueuedQueryEvent {}

#[derive(Clone, Debug, Default)]
pub struct SlashCommandRequest;

pub const FORK_PREFIX: &str = "";

pub use crate::ai::execution_profiles::CommandExecutionPermissionAllowedReason;

pub use history_model::AIQueryHistory;

pub struct StartAgentRequestId(uuid::Uuid);

pub fn inherit_child_agent_settings() {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AIBlockResponseRating {
    #[default]
    Unknown,
}

pub struct SessionContext;

pub struct ReadFileContextResult;

pub fn read_local_file_context(_path: &str) -> ReadFileContextResult {
    ReadFileContextResult
}

lazy_static! {
    pub static ref BLOCK_CONTEXT_ATTACHMENT_REGEX: Regex = Regex::new("").expect("valid regex");
    pub static ref DIFF_HUNK_ATTACHMENT_REGEX: Regex = Regex::new("").expect("valid regex");
    pub static ref DRIVE_OBJECT_ATTACHMENT_REGEX: Regex = Regex::new("").expect("valid regex");
}

pub fn ai_indicator_height(_app: &AppContext) -> f32 {
    0.0
}

pub fn render_ai_agent_mode_icon(_app: &AppContext) -> Option<Box<dyn warpui::elements::Element>> {
    None
}

pub fn render_ai_follow_up_icon(_app: &AppContext) -> Option<Box<dyn warpui::elements::Element>> {
    None
}

pub struct StartAgentRequest;
pub struct ClientIdentifiers;

lazy_static::lazy_static! {
    pub static ref CLAUDE_ORANGE: pathfinder_color::ColorU =
        pathfinder_color::ColorU::new(0, 0, 0, 255);
}

pub fn format_credits(_credits: f32) -> String {
    String::new()
}

pub fn apply_child_agent_model_override(
    _child_surface_id: warpui::EntityId,
    _model_id: Option<&str>,
    _ctx: &mut AppContext,
) {
}

#[cfg(not(target_family = "wasm"))]
pub fn prepare_local_oz_child_launch(
    _name: &str,
    _prompt: &str,
    _parent_run_id: Option<&str>,
    _ctx: &AppContext,
) -> impl std::future::Future<Output = anyhow::Result<PreparedLocalOzChildLaunch>> + 'static {
    async { Err(anyhow::anyhow!("prepare_local_oz_child_launch unavailable in warp_lite")) }
}

#[cfg(not(target_family = "wasm"))]
pub struct PreparedLocalOzChildLaunch {
    pub task_id: crate::ai::ambient_agents::AmbientAgentTaskId,
    pub conversation_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationStatusUpdate {
    Restored,
    Changed {
        prev_status: crate::ai::agent::conversation::ConversationStatus,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RequestFileEditsFormatKind {
    #[default]
    Unknown,
    StrReplace,
    V4A,
    Mixed,
}

pub use action_model::{
    ShellCommandExecutor, ShellCommandExecutorEvent, StartAgentExecutor, StartAgentExecutorEvent,
};
pub use conversation_selection::{
    ConversationSelection, ConversationSelectionEvent, ConversationSelectionHandle,
    PendingQueryState,
};

pub use block::{AIBlock, AIBlockAction, AIBlockEvent, TextLocation};
