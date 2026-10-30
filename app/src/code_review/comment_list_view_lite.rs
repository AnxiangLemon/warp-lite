//! Minimal comment list view stub for warp_lite builds.

use warpui::elements::{Element, Empty, ParentElement};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext, ViewHandle, WeakViewHandle};

use crate::code::buffer_location::LocalOrRemotePath;
use crate::code::editor::EditorCommentsModel;
use crate::code_review::code_review_view::CodeReviewView;
use crate::code_review::comments::{CommentId, ReviewCommentBatch};
use crate::workspace::view::right_panel::ReviewDestination;

#[derive(Clone, Debug, PartialEq)]
pub enum CommentListAction {}

#[derive(Clone, Debug)]
pub enum CommentListEvent {
    Submitted,
    Cancelled,
    DeleteComment { comment_id: CommentId },
    EditComment(CommentId),
    JumpToCommentLocation(CommentId),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommentListDebugState {
    pub review_destination: ReviewDestination,
    pub total_comments: usize,
    pub sendable_comments: usize,
    pub is_collapsed: bool,
    pub is_outdated_section_collapsed: Option<bool>,
    pub ai_available: bool,
    pub ai_enabled: bool,
    pub send_button_tooltip_text: String,
}

impl CommentListDebugState {
    fn default_state() -> Self {
        Self {
            review_destination: ReviewDestination::None,
            total_comments: 0,
            sendable_comments: 0,
            is_collapsed: false,
            is_outdated_section_collapsed: None,
            ai_available: false,
            ai_enabled: false,
            send_button_tooltip_text: String::new(),
        }
    }
}

pub struct CommentListView {}

impl CommentListView {
    pub fn new(
        _initial_repo_path: Option<LocalOrRemotePath>,
        _parent: WeakViewHandle<CodeReviewView>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {}
    }

    pub fn set_review_destination(
        &mut self,
        _destination: ReviewDestination,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_comment_model(
        &mut self,
        _model: Option<ModelHandle<ReviewCommentBatch>>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn debug_state(&self, _ctx: &AppContext) -> CommentListDebugState {
        CommentListDebugState::default_state()
    }

    pub fn can_send(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn scroll_to_comment(&mut self, _comment_id: CommentId, _ctx: &mut ViewContext<Self>) {}

    pub fn expand(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for CommentListView {
    type Event = CommentListEvent;
}

impl View for CommentListView {
    fn ui_name() -> &'static str {
        "CommentListView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CommentListView {
    type Action = CommentListAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub type CommentListViewHandle = ViewHandle<CommentListView>;
