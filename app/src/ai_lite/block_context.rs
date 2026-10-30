#[derive(Clone, Debug, Default)]
pub struct BlockContext;

pub fn block_context_from_terminal_model(
    _model: &crate::terminal::model::TerminalModel,
    _app: &warpui::AppContext,
) -> BlockContext {
    BlockContext
}
