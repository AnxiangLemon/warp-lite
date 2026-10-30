use warpui::elements::{Element, Empty};
use warpui::{Entity, ModelHandle, TypedActionView, View, ViewContext, ViewHandle};

use super::PanelPosition;
use super::global_search::view::GlobalSearchEntryFocus;
use crate::coding_panel_enablement_state::CodingPanelEnablementState;
use crate::pane_group::{PaneGroup, WorkingDirectoriesModel};

#[derive(Clone, Debug)]
pub enum LeftPanelAction {
    ProjectExplorer,
    GlobalSearch { entry_focus: GlobalSearchEntryFocus },
    WarpDrive,
    ConversationListView,
}

pub enum LeftPanelEvent {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolPanelView {
    ProjectExplorer,
    GlobalSearch { entry_focus: GlobalSearchEntryFocus },
    WarpDrive,
    ConversationListView,
}

/// Keeps workspace actions and stored panel identifiers compatible without constructing tool views.
pub struct LeftPanelView;

impl LeftPanelView {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        views: Vec<ToolPanelView>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let _ = (working_directories_model, views, ctx);
        Self
    }

    pub fn set_agent_management_view_open(&mut self, is_open: bool, ctx: &mut ViewContext<Self>) {
        let _ = (is_open, ctx);
    }

    pub fn set_panel_position(&mut self, position: PanelPosition, ctx: &mut ViewContext<Self>) {
        let _ = (position, ctx);
    }

    pub fn update_available_views(
        &mut self,
        views: Vec<ToolPanelView>,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = (views, ctx);
    }

    pub fn active_view(&self) -> ToolPanelView {
        ToolPanelView::ProjectExplorer
    }

    pub fn is_warp_drive_active(&self) -> bool {
        false
    }

    pub fn is_file_tree_active(&self) -> bool {
        false
    }

    pub(crate) fn auto_expand_active_file_tree_to_most_recent_directory(
        &mut self,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = ctx;
    }

    pub fn restore_active_view_from_snapshot(
        &mut self,
        view: ToolPanelView,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = (view, ctx);
    }

    pub fn set_active_pane_group(
        &mut self,
        pane_group: ViewHandle<PaneGroup>,
        working_directories_model: &ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = (pane_group, working_directories_model, ctx);
    }

    pub fn update_coding_panel_enablement(
        &mut self,
        enablement: CodingPanelEnablementState,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = (enablement, ctx);
    }

    pub fn focus_active_view_on_entry(&mut self, ctx: &mut ViewContext<Self>) {
        let _ = ctx;
    }

    pub fn handle_action_with_force_open(
        &mut self,
        action: &LeftPanelAction,
        force_open: bool,
        ctx: &mut ViewContext<Self>,
    ) {
        let _ = (action, force_open, ctx);
    }

    pub fn on_left_panel_visibility_changed(&self, is_now_open: bool, ctx: &mut ViewContext<Self>) {
        let _ = (is_now_open, ctx);
    }
}

impl Entity for LeftPanelView {
    type Event = LeftPanelEvent;
}

impl TypedActionView for LeftPanelView {
    type Action = LeftPanelAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        self.handle_action_with_force_open(action, false, ctx);
    }
}

impl View for LeftPanelView {
    fn ui_name() -> &'static str {
        "LeftPanelView"
    }

    fn render(&self, app: &warpui::AppContext) -> Box<dyn Element> {
        let _ = app;
        Empty::new().finish()
    }
}
