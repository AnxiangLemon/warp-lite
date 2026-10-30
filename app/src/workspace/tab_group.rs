//! Tab group data model. Gated at runtime by `FeatureFlag::GroupedTabs`.

use uuid::Uuid;
use warpui::elements::DraggableState;

use crate::tab::SelectedTabColor;

/// Stable identity for a tab group.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TabGroupId(pub Uuid);

impl TabGroupId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TabGroupId {
    fn default() -> Self {
        Self::new()
    }
}

/// A named group of tabs in the vertical tabs panel.
/// Member tabs reference their group via `TabData::group_id`.
#[derive(Clone)]
pub struct TabGroup {
    pub id: TabGroupId,
    pub name: Option<String>,
    pub color: SelectedTabColor,
    pub collapsed: bool,
    pub draggable_state: DraggableState,
    /// True when this whole group is pinned to the front of the tab list.
    pub pinned: bool,
}

impl TabGroup {
    /// Creates a new, untitled, expanded tab group with a fresh id.
    pub fn new() -> Self {
        Self {
            id: TabGroupId::new(),
            name: None,
            color: SelectedTabColor::default(),
            collapsed: false,
            draggable_state: Default::default(),
            pinned: false,
        }
    }
}

impl Default for TabGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "warp_lite")]
pub(crate) fn directory_group_name(directory: &str) -> Option<String> {
    let directory = directory.trim();
    if directory.is_empty() {
        return None;
    }
    let trimmed = directory.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() || (trimmed.len() == 2 && trimmed.ends_with(':')) {
        return Some(directory.to_string());
    }
    trimmed.rsplit(['/', '\\']).next().map(str::to_string)
}

#[cfg(all(test, feature = "warp_lite"))]
#[path = "tab_group_tests.rs"]
mod tests;
