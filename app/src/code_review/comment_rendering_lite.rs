//! Minimal comment rendering stubs for warp_lite builds.

use std::rc::Rc;

use warpui::elements::{MouseStateHandle, ParentElement};
use warpui::units::Pixels;
use warpui::{AppContext, Element, EventContext, View, ViewContext};

use crate::code::buffer_location::LocalOrRemotePath;
use crate::code_review::comments::AttachedReviewComment;

pub(crate) struct HeaderClickHandler {
    pub mouse_state: MouseStateHandle,
    pub on_click: Rc<dyn Fn(&mut EventContext) + 'static>,
}

pub(crate) struct CommentViewCard {
    source: AttachedReviewComment,
    is_collapsed: bool,
}

impl CommentViewCard {
    pub(crate) fn new<V: View>(
        source: AttachedReviewComment,
        _always_use_static_diff: bool,
        _disable_scrolling: bool,
        _max_width: Option<Pixels>,
        _repo_path: Option<&LocalOrRemotePath>,
        _ctx: &mut ViewContext<V>,
    ) -> Self {
        Self {
            source,
            is_collapsed: false,
        }
    }

    pub(crate) fn toggle_collapsed(&mut self) {
        self.is_collapsed = !self.is_collapsed;
    }

    pub(crate) fn is_collapsed(&self) -> bool {
        self.is_collapsed
    }

    pub(crate) fn source(&self) -> &AttachedReviewComment {
        &self.source
    }

    pub(crate) fn update_source<V: View>(
        &mut self,
        new_source: AttachedReviewComment,
        _repo_path: Option<&LocalOrRemotePath>,
        _ctx: &mut ViewContext<V>,
    ) {
        self.source = new_source;
    }

    pub(crate) fn render(
        &self,
        _editor_lens_element: Option<Box<dyn warpui::Element>>,
        _header_trailing_element: Option<Box<dyn warpui::Element>>,
        _metadata_trailing_element: Option<Box<dyn warpui::Element>>,
        _on_header_click: Option<&HeaderClickHandler>,
        _app: &AppContext,
    ) -> Box<dyn warpui::Element> {
        warpui::elements::Empty::new().finish()
    }
}
