#[cfg(feature = "tree_sitter_types")]
use std::iter;
#[cfg(feature = "tree_sitter_types")]
use std::ops::Range;

#[cfg(feature = "tree_sitter_types")]
use arborium::tree_sitter::{Node, TextProvider};
use warp_editor::content::version::BufferVersion;
use warpui_core::color::ColorU;

/// Color mapping from parsed syntax token name to its corresponding highlighting color.
#[derive(Clone, Copy)]
pub struct ColorMap {
    pub keyword_color: ColorU,
    pub function_color: ColorU,
    pub string_color: ColorU,
    pub type_color: ColorU,
    pub number_color: ColorU,
    pub comment_color: ColorU,
    pub property_color: ColorU,
    pub tag_color: ColorU,
}

pub struct TextSlice<'a>(pub &'a [u8]);

#[cfg(feature = "tree_sitter_types")]
impl TextSlice<'_> {
    pub(crate) fn get(&self, range: Range<usize>) -> Self {
        Self(self.0.get(range).unwrap_or_default())
    }
}

impl AsRef<[u8]> for TextSlice<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

/// The absolute indentation unit from start of the line.
#[derive(Debug)]
pub struct IndentDelta {
    pub delta: u8,
}

pub enum DecorationStateEvent {
    DecorationUpdated { version: BufferVersion },
}

#[cfg(feature = "tree_sitter_types")]
impl<'a> TextProvider<TextSlice<'a>> for TextSlice<'a> {
    type I = iter::Once<TextSlice<'a>>;

    fn text(&mut self, node: Node) -> Self::I {
        iter::once(self.get(node.byte_range()))
    }
}
