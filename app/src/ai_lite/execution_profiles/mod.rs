use uuid::Uuid;
use warpui::{Entity, ModelContext, SingletonEntity};

use crate::LaunchMode;
use settings_value::SettingsValue;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExecutionProfileId(Uuid);

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionProfilesConfig;

impl SettingsValue for ExecutionProfilesConfig {}

pub struct AIExecutionProfile;
pub struct CloudAIExecutionProfile;
pub struct CloudAIExecutionProfileModel;

pub struct CloudAgentComputerUseState {
    pub enabled: bool,
    pub forced_by_org: bool,
}

pub fn resolve_cloud_agent_computer_use_state(
    _ctx: &warpui::AppContext,
) -> CloudAgentComputerUseState {
    CloudAgentComputerUseState {
        enabled: false,
        forced_by_org: false,
    }
}

pub mod model_menu_items {
    use warpui::{Action, AppContext};

    use crate::ai::llms::LLMInfo;
    use crate::menu::{MenuItem, MenuItemFields};

    pub struct ModelMenuItem;

    pub fn is_auto(_llm: &LLMInfo) -> bool {
        false
    }

    pub fn has_reasoning_variants(_llm: &LLMInfo, _all_models: &[&LLMInfo]) -> bool {
        false
    }

    pub fn available_model_menu_items<A: Action + Clone>(
        _models: &[&LLMInfo],
        _profile_default_model: Option<&crate::ai::llms::LLMId>,
        _ctx: &AppContext,
    ) -> Vec<MenuItem<A>> {
        vec![]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionPermission {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WriteToPtyPermission {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AskUserQuestionPermission {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CommandExecutionPermissionAllowedReason {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComputerUsePermission {
    #[default]
    Unknown,
}

pub mod editor {
    use warpui::elements::{Element, Empty};
    use warpui::{Action, AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext};

    use crate::menu::MenuItem;
    use crate::pane_group::focus_state::PaneFocusHandle;
    use crate::pane_group::pane::{BackingView, view};
    use crate::pane_group::PaneEvent;

    #[derive(Clone, Debug)]
    pub enum CustomRouterEditorEvent {}

    #[derive(Clone, Debug)]
    pub enum ExecutionProfileEditorViewEvent {
        Close,
    }

    #[derive(Clone, Debug)]
    pub enum ExecutionProfileEditorViewAction {}

    pub struct ExecutionProfileEditorManager;

    impl ExecutionProfileEditorManager {
        pub fn default() -> Self {
            Self
        }
    }

    impl Entity for ExecutionProfileEditorManager {
        type Event = ();
    }

    impl SingletonEntity for ExecutionProfileEditorManager {}

    pub struct ExecutionProfileEditorView {
        focus_handle: Option<PaneFocusHandle>,
    }

    impl Entity for ExecutionProfileEditorView {
        type Event = ExecutionProfileEditorViewEvent;
    }

    impl View for ExecutionProfileEditorView {
        fn ui_name() -> &'static str {
            "ExecutionProfileEditorViewLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl TypedActionView for ExecutionProfileEditorView {
        type Action = ExecutionProfileEditorViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl BackingView for ExecutionProfileEditorView {
        type PaneHeaderOverflowMenuAction = ExecutionProfileEditorViewAction;
        type CustomAction = ();
        type AssociatedData = ();

        fn handle_pane_header_overflow_menu_action(
            &mut self,
            _action: &Self::PaneHeaderOverflowMenuAction,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        fn close(&mut self, ctx: &mut ViewContext<Self>) {
            ctx.emit(ExecutionProfileEditorViewEvent::Close);
        }

        fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
            ctx.focus_self();
        }

        fn pane_header_overflow_menu_items(
            &self,
            _ctx: &AppContext,
        ) -> Vec<MenuItem<Self::PaneHeaderOverflowMenuAction>> {
            vec![]
        }

        fn render_header_content(
            &self,
            _ctx: &view::HeaderRenderContext<'_>,
            _app: &AppContext,
        ) -> view::HeaderContent {
            view::HeaderContent::Standard(view::StandardHeader {
                title: "Execution Profile".to_string(),
                title_secondary: None,
                title_style: None,
                title_clip_config: warpui::text_layout::ClipConfig::start(),
                title_max_width: None,
                left_of_title: None,
                right_of_title: None,
                left_of_overflow: None,
                options: view::StandardHeaderOptions::default(),
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

pub mod profiles {
    use warpui::{Entity, ModelContext, SingletonEntity};

    use crate::LaunchMode;

    #[derive(Clone, Debug)]
    pub enum AIExecutionProfilesModelEvent {}

    pub struct AIExecutionProfilesModel;

    impl AIExecutionProfilesModel {
        pub fn new(_launch_mode: LaunchMode, _ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for AIExecutionProfilesModel {
        type Event = AIExecutionProfilesModelEvent;
    }

    impl SingletonEntity for AIExecutionProfilesModel {}
}

pub use editor::ExecutionProfileEditorManager;
pub use profiles::AIExecutionProfilesModel;
