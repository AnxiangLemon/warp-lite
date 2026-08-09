#![allow(clippy::doc_lazy_continuation)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

#[cfg(feature = "warp_lite")]
use pathfinder_geometry::vector::vec2f;
#[cfg(feature = "warp_lite")]
use warpui::platform::WindowBounds;

pub static ASSETS: warp_assets::Assets = warp_assets::Assets;

#[cfg(feature = "warp_lite")]
mod lite;
#[cfg(all(feature = "warp_lite", target_family = "wasm"))]
#[path = "platform/mod.rs"]
mod platform;

#[cfg(feature = "warp_lite")]
pub fn run_lite() -> anyhow::Result<()> {
    #[cfg(target_family = "wasm")]
    platform::init();

    let app_builder = warpui::platform::AppBuilder::new(
        warpui::platform::AppCallbacks::default(),
        Box::new(ASSETS),
        None,
    );

    app_builder.run(|ctx| {
        let window_options = warpui::AddWindowOptions {
            window_bounds: WindowBounds::ExactSize(vec2f(1400.0, 900.0)),
            window_style: warpui::platform::WindowStyle::Normal,
            ..Default::default()
        };
        ctx.add_window(window_options, lite::LiteRootView::new);
    })
}

#[cfg(not(feature = "warp_lite"))]
include!("lib_full.rs");
