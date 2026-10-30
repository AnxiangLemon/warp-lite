use warpui::elements::{Element, Empty, MouseStateHandle};
use warpui::{AppContext, EventContext};

pub struct CodeBlockOptions;

#[derive(Default, Clone)]
pub struct CodeSnippetButtonHandles {
    pub open_button: MouseStateHandle,
    pub copy_button: MouseStateHandle,
    pub insert_button: MouseStateHandle,
}

impl CodeSnippetButtonHandles {
    pub fn reset_hover_state_on_focus_change(&self) {}
}

pub type HandleCode = Box<dyn FnMut(String, &mut EventContext)>;

pub fn render_code_block_plain(
    _app: &AppContext,
    _code: &str,
    _language: Option<&str>,
    _options: CodeBlockOptions,
) -> Box<dyn Element> {
    Empty::new().finish()
}

pub fn render_runnable_code_snippet(
    _app: &AppContext,
    _code: &str,
    _language: Option<&str>,
    _button_handles: CodeSnippetButtonHandles,
    _on_run: HandleCode,
) -> Box<dyn Element> {
    Empty::new().finish()
}
