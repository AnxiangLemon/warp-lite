//! Lite stubs for AI-only inline input menus.

use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, EntityId, ModelHandle, View, ViewContext, ViewHandle};

use crate::ai::agent::AIAgentExchangeId;
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent_conversations_model::AgentConversationEntryId;
use crate::ai::blocklist::agent_view::AgentViewController;
use crate::ai::blocklist::block::cli_controller::CLISubagentController;
use crate::ai::blocklist::conversation_selection::ConversationSelectionHandle;
use crate::ai::execution_profiles::ExecutionProfileId;
use crate::ai::llms::LLMId;
use crate::terminal::input::buffer_model::InputBufferModel;
use crate::terminal::input::inline_menu::InlineMenuPositioner;
use crate::terminal::input::suggestions_mode_model::InputSuggestionsModeModel;
use crate::terminal::model::session::active_session::ActiveSession;
use crate::terminal::view::ambient_agent::AmbientAgentViewModel;

// --- conversations ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineConversationMenuTab {
    All,
    CurrentDirectory,
}

#[derive(Clone, Debug)]
pub struct AcceptConversation {
    pub item_id: AgentConversationEntryId,
    pub is_open_elsewhere: bool,
}

#[derive(Debug, Clone)]
pub enum InlineConversationMenuEvent {
    NavigateToConversation {
        item_id: AgentConversationEntryId,
    },
    Dismissed,
}

pub struct InlineConversationMenuView;

impl InlineConversationMenuView {
    pub fn new(
        _input_suggestions_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _conversation_selection: ConversationSelectionHandle,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _active_session: ModelHandle<ActiveSession>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn select_next_tab(&self, _ctx: &mut ViewContext<Self>) -> bool {
        false
    }

    pub fn select_up(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for InlineConversationMenuView {
    type Event = InlineConversationMenuEvent;
}

impl View for InlineConversationMenuView {
    fn ui_name() -> &'static str {
        "InlineConversationMenuViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

// --- models ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineModelSelectorTab {
    BaseAgent,
    FullTerminalUse,
}

#[derive(Debug, Clone)]
pub enum InlineModelSelectorEvent {
    SelectedModel {
        id: LLMId,
        selected_tab: InlineModelSelectorTab,
        set_as_default: bool,
    },
    Dismissed,
}

pub struct InlineModelSelectorView {
    filter_results_by_input: bool,
    prompt_parked_for_search: bool,
}

impl InlineModelSelectorView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _terminal_view_id: EntityId,
        _ambient_agent_view_model: Option<ModelHandle<AmbientAgentViewModel>>,
        _suggestions_mode_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _cli_subagent_controller: ModelHandle<CLISubagentController>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            filter_results_by_input: false,
            prompt_parked_for_search: false,
        }
    }

    pub fn set_ambient_agent_view_model(
        &mut self,
        _view_model: ModelHandle<AmbientAgentViewModel>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn filter_results_by_input(&self) -> bool {
        self.filter_results_by_input
    }

    pub fn set_filter_results_by_input(&mut self, filter: bool) {
        self.filter_results_by_input = filter;
    }

    pub fn prompt_parked_for_search(&self) -> bool {
        self.prompt_parked_for_search
    }

    pub fn set_prompt_parked_for_search(&mut self, parked: bool) {
        self.prompt_parked_for_search = parked;
    }

    pub fn set_active_tab(&self, _tab: InlineModelSelectorTab, _ctx: &mut ViewContext<Self>) {}

    pub fn select_next_tab(&self, _ctx: &mut ViewContext<Self>) -> bool {
        false
    }

    pub fn select_up(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&self, _cmd_or_ctrl_enter: bool, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for InlineModelSelectorView {
    type Event = InlineModelSelectorEvent;
}

impl View for InlineModelSelectorView {
    fn ui_name() -> &'static str {
        "InlineModelSelectorViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

// --- profiles ---

#[derive(Debug, Clone)]
pub enum InlineProfileSelectorEvent {
    SelectedProfile { profile_id: ExecutionProfileId },
    ManageProfiles,
    Dismissed,
}

pub struct InlineProfileSelectorView;

impl InlineProfileSelectorView {
    pub fn new(
        _terminal_view_id: EntityId,
        _suggestions_mode_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn select_up(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for InlineProfileSelectorView {
    type Event = InlineProfileSelectorEvent;
}

impl View for InlineProfileSelectorView {
    fn ui_name() -> &'static str {
        "InlineProfileSelectorViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

// --- rewind ---

#[derive(Clone, Debug)]
pub struct SelectRewindPoint;

#[derive(Debug, Clone)]
pub enum RewindMenuEvent {
    AcceptedRewindPoint {
        exchange_id: Option<AIAgentExchangeId>,
    },
    Dismissed,
}

pub struct RewindMenuView;

impl RewindMenuView {
    pub fn new(
        _conversation_id: AIConversationId,
        _input_suggestions_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn select_up(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for RewindMenuView {
    type Event = RewindMenuEvent;
}

impl View for RewindMenuView {
    fn ui_name() -> &'static str {
        "RewindMenuViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

// --- user_query ---

#[derive(Clone, Debug)]
pub struct SelectUserQuery;

#[derive(Debug, Clone)]
pub enum UserQueryMenuEvent {
    SelectedQuery { exchange_id: AIAgentExchangeId },
    AcceptedQuery {
        exchange_id: AIAgentExchangeId,
        cmd_enter: bool,
    },
    Dismissed,
}

pub struct UserQueryMenuView;

impl UserQueryMenuView {
    pub fn new(
        _conversation_id: AIConversationId,
        _input_suggestions_model: ModelHandle<InputSuggestionsModeModel>,
        _agent_view_controller: ModelHandle<AgentViewController>,
        _positioner: &ModelHandle<InlineMenuPositioner>,
        _input_buffer_model: &ModelHandle<InputBufferModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn select_up(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_down(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn accept_selected_item(&self, _cmd_enter: bool, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for UserQueryMenuView {
    type Event = UserQueryMenuEvent;
}

impl View for UserQueryMenuView {
    fn ui_name() -> &'static str {
        "UserQueryMenuViewLite"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
