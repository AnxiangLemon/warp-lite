use std::borrow::Cow;
#[cfg(all(feature = "lite", not(target_family = "wasm")))]
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use rust_embed::RustEmbed;
#[cfg(all(feature = "lite", target_os = "windows"))]
use rust_embed_compressed as rust_embed;
use warpui_core::AssetProvider;

#[derive(Clone, Copy, RustEmbed)]
#[folder = "../../app/assets"]
#[include = "bundled/**"] // Should be kept in sync with BUNDLED_ASSETS_DIR.
#[include = "async/**"] // Should be kept in sync with ASYNC_ASSETS_DIR.
#[cfg_attr(feature = "lite", exclude = "async/png/**")]
#[cfg_attr(
    all(feature = "lite", target_os = "windows"),
    exclude = "bundled/png/**"
)]
#[cfg_attr(
    all(feature = "lite", target_os = "windows"),
    exclude = "bundled/syntax_theme/**"
)]
#[cfg_attr(
    all(feature = "lite", not(target_family = "wasm")),
    exclude = "async/jpg/**"
)]
#[cfg_attr(
    all(feature = "lite", target_os = "windows"),
    exclude = "bundled/fonts/**/LICENSE*"
)]
#[cfg_attr(target_family = "wasm", exclude = "async/**")]
// Excludes take precedence.
// Standalone CLI builds (the `oz` tarball) are headless and never render the
// onboarding/theme imagery in `async/`, so we exclude those bytes from the
// embedded asset set to keep the CLI binary small — mirroring the carve-out
// already applied for the WASM target above.
#[cfg_attr(feature = "standalone", exclude = "async/**")]
pub struct Assets;

impl AssetProvider for Assets {
    fn get(&self, path: &str) -> Result<Cow<'_, [u8]>> {
        #[cfg(all(feature = "lite", not(target_family = "wasm")))]
        if path.starts_with("async/jpg/") {
            let executable = std::env::current_exe()?;
            let image = external_background_path(&executable, path)
                .ok_or_else(|| anyhow!("invalid external background asset path"))?;
            return std::fs::read(image).map(Cow::Owned).map_err(Into::into);
        }
        <Assets as RustEmbed>::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("no asset exists at path {path}"))
    }
}

#[cfg(all(feature = "lite", not(target_family = "wasm")))]
pub fn background_directory() -> Result<PathBuf> {
    let executable = std::env::current_exe()?;
    background_directory_for_executable(&executable)
}

#[cfg(all(feature = "lite", not(target_family = "wasm")))]
fn background_directory_for_executable(executable: &Path) -> Result<PathBuf> {
    executable
        .parent()
        .filter(|parent| parent.is_absolute())
        .map(|parent| parent.join("backgrounds"))
        .ok_or_else(|| anyhow!("executable has no parent directory"))
}

#[cfg(all(feature = "lite", not(target_family = "wasm")))]
fn external_background_path(executable: &Path, asset: &str) -> Option<PathBuf> {
    let name = asset.strip_prefix("async/jpg/")?;
    if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\', ':', '\0']) {
        return None;
    }
    Some(
        background_directory_for_executable(executable)
            .ok()?
            .join(name),
    )
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
