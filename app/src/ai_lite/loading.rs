use warpui::elements::{Element, Empty, ParentElement, shimmering_text::ShimmeringTextStateHandle};
use warpui::AppContext;

pub fn shimmering_warp_loading_text(
    _text: impl Into<String>,
    _font_size: f32,
    _shimmer_handle: ShimmeringTextStateHandle,
    _app: &AppContext,
) -> Box<dyn Element> {
    Empty::new().finish()
}
