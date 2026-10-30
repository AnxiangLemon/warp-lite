use warpui::elements::{Element, Empty};
use warpui::{Action, AppContext, Entity, TypedActionView, View, ViewContext};

use crate::menu::MenuItem;
use crate::pane_group::focus_state::PaneFocusHandle;
use crate::pane_group::pane::{BackingView, view};
use crate::pane_group::PaneEvent;

    #[derive(Clone, Debug)]
    pub enum CustomRouterEditorEvent {
        Close,
    }

pub struct CustomModelRouterEditor;

pub type CustomRouterEditorView = editor::CustomModelRouterEditorView;

pub use editor::CustomModelRouterEditorView;

pub mod editor {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum CustomRouterEditorViewAction {}

    pub struct CustomModelRouterEditorView {
        focus_handle: Option<PaneFocusHandle>,
    }

    impl Entity for CustomModelRouterEditorView {
        type Event = CustomRouterEditorEvent;
    }

    impl View for CustomModelRouterEditorView {
        fn ui_name() -> &'static str {
            "CustomModelRouterEditorViewLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl TypedActionView for CustomModelRouterEditorView {
        type Action = CustomRouterEditorViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl BackingView for CustomModelRouterEditorView {
        type PaneHeaderOverflowMenuAction = CustomRouterEditorViewAction;
        type CustomAction = ();
        type AssociatedData = ();

        fn handle_pane_header_overflow_menu_action(
            &mut self,
            _action: &Self::PaneHeaderOverflowMenuAction,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        fn close(&mut self, ctx: &mut ViewContext<Self>) {
            ctx.emit(CustomRouterEditorEvent::Close);
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
                title: "Custom Router".to_string(),
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
