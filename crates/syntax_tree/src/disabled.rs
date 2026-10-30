use std::cell::Ref;
use std::sync::Arc;

use languages::Language;
use rangemap::{RangeMap, RangeSet};
use string_offset::CharOffset;
use warp_editor::content::buffer::{Buffer, BufferSnapshot};
use warp_editor::content::edit::PreciseDelta;
use warp_editor::content::text::IndentUnit;
use warp_editor::content::version::BufferVersion;
use warp_editor::decoration::DecorationLayer;
use warpui_core::color::ColorU;
use warpui_core::text::point::Point;
use warpui_core::{AppContext, Entity, ModelContext, WeakModelHandle};

use crate::{ColorMap, DecorationStateEvent, IndentDelta};

/// An unavailable parser capability; plain text editing and independent decorations still work.
pub struct SyntaxTreeState;

impl SyntaxTreeState {
    pub fn new(
        buffer_handle: WeakModelHandle<Buffer>,
        buffer_version: BufferVersion,
        color_map: ColorMap,
    ) -> Self {
        let _ = (buffer_handle, buffer_version, color_map);
        Self
    }

    pub fn set_language(&mut self, language: Arc<Language>) {
        drop(language);
    }

    pub fn has_supported_highlighting(&self) -> bool {
        false
    }

    pub fn indent_unit(&self) -> Option<IndentUnit> {
        None
    }

    pub fn bracket_pairs(&self) -> Option<&[(char, char)]> {
        None
    }

    pub fn comment_prefix(&self) -> Option<&str> {
        None
    }

    pub fn highlights_in_ranges(
        &self,
        ranges: RangeSet<CharOffset>,
        render_content_version: Option<BufferVersion>,
        ctx: &AppContext,
    ) -> Option<Ref<'_, RangeMap<CharOffset, ColorU>>> {
        drop(ranges);
        let _ = (render_content_version, ctx);
        None
    }

    pub fn indentation_at_point(&self, point: Point, ctx: &AppContext) -> Option<IndentDelta> {
        let _ = (point, ctx);
        None
    }

    pub fn invalidate_highlight_cache_for_version(&self, version: BufferVersion) {
        let _ = version;
    }

    pub fn set_color_map(&mut self, color_map: ColorMap) {
        let _ = color_map;
    }
}

impl DecorationLayer for SyntaxTreeState {
    fn update_internal_state_with_delta(
        &mut self,
        deltas: &[PreciseDelta],
        version: BufferVersion,
        content: BufferSnapshot,
        ctx: &mut ModelContext<Self>,
    ) {
        drop(content);
        let _ = (deltas, version, ctx);
    }
}

impl Entity for SyntaxTreeState {
    type Event = DecorationStateEvent;
}
