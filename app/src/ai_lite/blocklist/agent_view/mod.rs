use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::elements::{Element, Empty};
use warpui::{
    AppContext, Entity, EntityId, ModelContext, ModelHandle, View, ViewContext,
};

use crate::terminal::model::TerminalModel;

pub mod agent_input_footer {
    use super::*;

    pub mod editor {
        pub use super::super::editor::{AgentToolbarEditorEvent, AgentToolbarEditorMode, AgentToolbarEditorModal};
    }

    pub mod toolbar_item {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum AgentToolbarItemKind {
            #[default]
            Unknown,
        }
    }

    pub struct AgentInputButtonTheme;

    #[derive(Clone, Debug)]
    pub enum AgentInputFooterEvent {
        SelectFile,
        StartRemoteControl,
        StopRemoteControl,
        WriteToPty(String),
        InsertIntoCLIPty(String),
        InsertIntoCLIRichInput(String),
        ToggleCodeReviewPane(bool),
        ToggleFileExplorer(bool),
        OpenRichInput,
        HideRichInput,
        ToggledChipMenu { open: bool },
        TryExecuteChipCommand(String),
        PromptAlert(super::super::prompt::prompt_alert::PromptAlertEvent),
        ModelSelectorOpened,
        ModelSelectorClosed,
        EnvironmentSelectorClosed,
        ToggleInlineModelSelector { initial_tab: () },
        OpenSettings(()),
        OpenCodeReview,
        OpenAIDocument {
            document_id: crate::ai::document::AIDocumentId,
            document_version: crate::ai::document::AIDocumentVersion,
        },
        ShowContextMenu { position: pathfinder_geometry::vector::Vector2F },
        OpenEnvironmentManagementPane,
        PluginInstalled(()),
        HandoffChipClicked,
    }

    pub struct AgentInputFooter;

    impl AgentInputFooter {
        #[allow(clippy::too_many_arguments)]
        pub fn new(
            _menu_positioning_provider: ModelHandle<
                crate::terminal::input::inline_menu::InlineMenuPositioner,
            >,
            _terminal_view_id: EntityId,
            _ai_input_model: ModelHandle<super::super::BlocklistAIInputModel>,
            _model: Arc<FairMutex<TerminalModel>>,
            _ambient_agent_view_model: Option<
                ModelHandle<crate::terminal::view::ambient_agent::AmbientAgentViewModel>,
            >,
            _handoff_compose_state: ModelHandle<
                crate::terminal::input::handoff_compose::HandoffComposeState,
            >,
            _current_prompt: ModelHandle<crate::context_chips::prompt_type::PromptType>,
            _display_chip_config: crate::context_chips::display_chip::DisplayChipConfig,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self
        }
    }

    impl Entity for AgentInputFooter {
        type Event = ();
    }

    impl View for AgentInputFooter {
        fn ui_name() -> &'static str {
            "AgentInputFooterLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub mod editor;

pub mod shortcuts {
    use warpui::{Entity, ModelContext};

    pub struct AgentShortcutViewModel;

    impl AgentShortcutViewModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for AgentShortcutViewModel {
        type Event = ();
    }

    pub fn render_keystroke_with_color_overrides(
        _keystroke: &str,
    ) -> Option<Box<dyn warpui::elements::Element>> {
        None
    }
}

pub fn is_in_cloud_context(_app: &warpui::AppContext) -> bool {
    false
}

pub struct AgentMessageBarMouseStates;
pub struct AgentViewEntryBlock;
pub struct AgentViewEntryBlockParams;

#[derive(Clone, Debug)]
pub enum AgentViewEntryBlockEvent {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AutoTriggerBehavior {
    #[default]
    Never,
    Always,
    InAgentView,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnterAgentViewError {
    #[default]
    Unknown,
}

pub struct AgentViewHeaderDisabledTheme;
pub struct AgentViewHeaderTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExitConfirmationTrigger {
    #[default]
    Unknown,
}

pub struct GuiInputModePolicy;

pub fn fork_from_last_known_good_state_exchange_id(
    _conversation_id: crate::ai::agent::conversation::AIConversationId,
) -> Option<crate::ai::agent::AIAgentExchangeId> {
    None
}

pub fn get_agent_view_entry_block_position_id(_block_id: warpui::EntityId) -> String {
    String::new()
}

pub use inline_agent_view_header::InlineAgentViewHeader;
pub use zero_state_block::{AgentViewZeroStateBlock, AgentViewZeroStateEvent};

pub fn render_orchestration_breadcrumbs(
    _app: &warpui::AppContext,
) -> Option<Box<dyn warpui::elements::Element>> {
    None
}
pub struct AgentViewState;
pub struct AgentViewConversationSelection;

pub const ENTER_AGENT_VIEW_NEW_CONVERSATION_KEYSTROKE: &str = "";

pub fn render_block_container() -> Option<Box<dyn warpui::elements::Element>> {
    None
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DismissalStrategy {
    #[default]
    Unknown,
}

pub const ENTER_OR_EXIT_CONFIRMATION_WINDOW: std::time::Duration =
    std::time::Duration::from_secs(0);

#[derive(Clone, Debug, Default)]
pub struct EphemeralMessage;

#[derive(Clone, Debug)]
pub enum AgentViewControllerEvent {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentViewDisplayMode {
    #[default]
    FullScreen,
    Inline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentViewEntryOrigin {
    #[default]
    Unknown,
    CreateEnvironment,
    SlashInit,
    ThirdPartyCloudAgent,
    ConversationListView,
    InlineCodeReview,
}

pub struct AgentViewController;

impl AgentViewController {
    pub fn new(
        _model: Arc<FairMutex<TerminalModel>>,
        _terminal_view_id: EntityId,
        _ephemeral_message_model: ModelHandle<EphemeralMessageModel>,
    ) -> Self {
        Self
    }

    pub fn is_active(&self) -> bool {
        false
    }
}

impl Entity for AgentViewController {
    type Event = AgentViewControllerEvent;
}

pub struct EphemeralMessageModel;

impl EphemeralMessageModel {
    pub fn new() -> Self {
        Self
    }
}

impl Entity for EphemeralMessageModel {
    type Event = ();
}

pub struct OrchestrationPillBar;

impl OrchestrationPillBar {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Entity for OrchestrationPillBar {
    type Event = ();
}

impl View for OrchestrationPillBar {
    fn ui_name() -> &'static str {
        "OrchestrationPillBarLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

pub mod orchestration_pill_bar_model {
    use std::collections::HashSet;

    use warpui::{Entity, ModelContext, SingletonEntity};

    use crate::ai::agent::conversation::AIConversationId;

    pub struct OrchestrationPillBarModel;

    impl OrchestrationPillBarModel {
        pub fn new(
            _initial_pinned: HashSet<AIConversationId>,
            _ctx: &mut ModelContext<Self>,
        ) -> Self {
            Self
        }
    }

    impl Entity for OrchestrationPillBarModel {
        type Event = ();
    }

    impl SingletonEntity for OrchestrationPillBarModel {}
}

pub mod inline_agent_view_header {
    use super::*;

    pub struct InlineAgentViewHeader;

    impl Entity for InlineAgentViewHeader {
        type Event = ();
    }

    impl View for InlineAgentViewHeader {
        fn ui_name() -> &'static str {
            "InlineAgentViewHeaderLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub mod zero_state_block {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum AgentViewZeroStateEvent {}

    pub struct AgentViewZeroStateBlock;

    impl Entity for AgentViewZeroStateBlock {
        type Event = AgentViewZeroStateEvent;
    }

    impl View for AgentViewZeroStateBlock {
        fn ui_name() -> &'static str {
            "AgentViewZeroStateBlockLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub use agent_input_footer::AgentInputFooter;
pub use agent_input_footer::AgentInputFooterEvent;
pub use agent_input_footer::toolbar_item;
pub use agent_input_footer::toolbar_item::AgentToolbarItemKind;

use super::{BlocklistAIActionModel, BlocklistAIInputModel};

pub fn pane_group_id_containing_terminal_view(
    _terminal_view_id: EntityId,
    _app: &AppContext,
) -> Option<EntityId> {
    None
}

pub mod orchestration_conversation_links {
    pub use super::pane_group_id_containing_terminal_view;

    pub fn parent_conversation_navigation_card() -> Option<Box<dyn warpui::elements::Element>> {
        None
    }
}
