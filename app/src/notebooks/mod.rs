pub mod active_notebook_data;
mod context_menu;
pub mod editor;
pub mod file;
pub mod link;
pub mod manager;
pub mod notebook;
mod styles;
pub mod telemetry;

use warpui::AppContext;

pub use crate::cloud_object::{
    CloudNotebook, CloudNotebookModel, NotebookId, NotebookLocation, SerializedNotebook,
};

/// Initialize notebooks-related keybindings.
pub fn init(app: &mut AppContext) {
    self::notebook::init(app);
    self::file::init(app);
    self::editor::view::init(app);
}

/// Translate a notebook's Markdown content into an external Markdown format.
///
/// This:
/// * Normalizes code block languages
/// * Includes extra context for embedded objects.
#[cfg_attr(not(feature = "local_fs"), allow(dead_code))]
pub fn export_notebook(data: &str, ctx: &AppContext) -> anyhow::Result<String> {
    use warp_editor::content::buffer::Buffer;
    use warp_editor::content::markdown::MarkdownStyle;

    // Parse the Markdown directly rather than using [`Buffer::from_markdown`] so that we can
    // report errors to the exporter.
    let parsed = markdown_parser::parse_markdown(data)?;
    Ok(Buffer::export_to_markdown(
        parsed,
        Some(editor::notebook_embedded_item_conversion),
        MarkdownStyle::Export {
            app_context: Some(ctx),
            should_not_escape_markdown_punctuation: false,
        },
    ))
}
