use std::borrow::Cow;

use anyhow::{Result, anyhow};
use rust_embed::RustEmbed;
use warpui_core::AssetProvider;

#[derive(Clone, Copy, RustEmbed)]
#[folder = "tests/data/"]
pub struct TestOnlyAssets;

pub static TEST_ONLY_ASSETS: TestOnlyAssets = TestOnlyAssets;

impl AssetProvider for TestOnlyAssets {
    fn get(&self, path: &str) -> Result<Cow<'_, [u8]>> {
        <Self as RustEmbed>::get(path)
            .map(|file| file.data)
            .ok_or_else(|| anyhow!("no asset exists at path {path}"))
    }
}
