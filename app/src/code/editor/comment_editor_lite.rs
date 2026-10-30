use warpui::elements::{Element, Empty, ParentElement};
use warpui::units::Pixels;
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext, ViewHandle};

use crate::code::editor::EditorCommentsModel;
use crate::code::editor::line::EditorLineLocation;
use crate::code_review::comments::{CommentId, CommentOrigin};

/// Default width of the comment editor, in pixels.
pub(crate) const DEFAULT_COMMENT_MAX_WIDTH: f32 = 750.0;

pub struct CommentEditor {}

#[derive(Debug, Clone)]
pub enum CommentEditorEvent {
    ContentChanged,
    CommentSaved {
        id: Option<CommentId>,
        comment_text: String,
        #[cfg_attr(not(feature = "local_fs"), allow(dead_code))]
        line: Option<EditorLineLocation>,
    },
    CloseEditor,
    DeleteComment {
        id: CommentId,
    },
}

impl CommentEditor {
    pub fn new(_ctx: &mut ViewContext<Self>, _comments: ModelHandle<EditorCommentsModel>) -> Self {
        Self {}
    }

    pub fn reopen_saved_comment(
        &mut self,
        _id: &CommentId,
        _line: Option<EditorLineLocation>,
        _comment_text: &str,
        _origin: &CommentOrigin,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn get_laid_out_size(&self) -> pathfinder_geometry::vector::Vector2F {
        pathfinder_geometry::vector::Vector2F::new(100.0, 24.0)
    }

    pub fn comment_text(&self, _app: &AppContext) -> String {
        String::new()
    }
}

impl Entity for CommentEditor {
    type Event = CommentEditorEvent;
}

impl View for CommentEditor {
    fn ui_name() -> &'static str {
        "CommentEditor"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for CommentEditor {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

/// Placeholder rich-text editor view used by code-review under warp_lite.
pub struct RichTextEditorViewStub;

impl Entity for RichTextEditorViewStub {
    type Event = ();
}

impl View for RichTextEditorViewStub {
    fn ui_name() -> &'static str {
        "RichTextEditorViewStub"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl RichTextEditorViewStub {
    pub fn selected_text(&self, _app: &AppContext) -> Option<String> {
        None
    }

    pub fn clear_text_selection(&mut self, _ctx: &mut ViewContext<Self>) {}
}

pub(crate) fn create_readonly_comment_markdown_editor<V: View>(
    _content: &str,
    _disable_scrolling: bool,
    _max_width: Option<Pixels>,
    ctx: &mut ViewContext<V>,
) -> ViewHandle<RichTextEditorViewStub> {
    ctx.add_view(|_ctx| RichTextEditorViewStub)
}
