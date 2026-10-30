#[path = "test/assets.rs"]
mod assets;
#[cfg(target_os = "windows")]
#[path = "test/lite.rs"]
mod lite;

pub use assets::{TEST_ONLY_ASSETS, TestOnlyAssets};
#[cfg(target_os = "windows")]
pub use lite::*;
