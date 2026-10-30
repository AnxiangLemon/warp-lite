use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::sync::{Arc, OnceLock};

use anyhow::{Result, anyhow, ensure};
use bytes::Bytes;
use image::codecs::png::PngDecoder;
use image::{DynamicImage, Limits};
use warpui::assets::asset_cache::{AssetSource, AsyncAssetId, AsyncAssetType};
use warpui::image_cache::{CustomImageFormat, CustomImageHeader};

use super::theme::{Image, WarpTheme};

const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_INPUT_DIMENSION: u32 = 4096;
const MAX_TEXTURE_DIMENSION: u32 = 2048;

struct StartupBackground;
impl AsyncAssetType for StartupBackground {}

fn load_background(path: &Path) -> Result<Image> {
    let file = File::open(path)?;
    ensure!(
        file.metadata()?.is_file(),
        "Background is not a regular file"
    );
    ensure!(
        file.metadata()?.len() <= MAX_FILE_BYTES,
        "Background file is too large"
    );
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= MAX_FILE_BYTES,
        "Background file is too large"
    );

    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_INPUT_DIMENSION);
    limits.max_image_height = Some(MAX_INPUT_DIMENSION);
    limits.max_alloc = Some(96 * 1024 * 1024);
    let decoder = PngDecoder::with_limits(Cursor::new(bytes), limits)?;
    ensure!(
        !decoder.is_apng()?,
        "Animated backgrounds are not supported"
    );
    let decoded = DynamicImage::from_decoder(decoder)?;
    let decoded =
        if decoded.width() > MAX_TEXTURE_DIMENSION || decoded.height() > MAX_TEXTURE_DIMENSION {
            decoded.thumbnail(MAX_TEXTURE_DIMENSION, MAX_TEXTURE_DIMENSION)
        } else {
            decoded
        };
    let rgba = decoded.into_rgba8();
    let (width, height) = rgba.dimensions();
    let bytes = CustomImageHeader::prepend_custom_header(
        rgba.into_raw(),
        width,
        height,
        CustomImageFormat::Rgba,
    )
    .map_err(|error| anyhow!("Invalid background bitmap: {error:?}"))?;
    let bytes = Bytes::from(bytes);

    // Freeze validated pixels so later file edits cannot change a live GPU asset.
    Ok(Image {
        source: AssetSource::Async {
            id: AsyncAssetId::new::<StartupBackground>("startup-background"),
            fetch: Arc::new(move || {
                let bytes = bytes.clone();
                Box::pin(async move { Ok(bytes) })
            }),
        },
        opacity: 30,
    })
}

fn startup_background() -> Option<Image> {
    static BACKGROUND: OnceLock<Option<Image>> = OnceLock::new();
    BACKGROUND
        .get_or_init(|| {
            // Recovery must not reload the optional image that may have caused a crash.
            if std::env::args_os().skip(1).any(|argument| {
                let argument = argument.to_string_lossy();
                argument == "--crash-recovery-mechanism"
                    || argument.starts_with("--crash-recovery-mechanism=")
            }) {
                return None;
            }
            #[cfg(feature = "integration_tests")]
            let directory = std::env::var_os("WARP_LITE_TEST_BACKGROUND_DIR")
                .map(std::path::PathBuf::from)
                .map(Ok)
                .unwrap_or_else(warp_assets::background_directory);
            #[cfg(not(feature = "integration_tests"))]
            let directory = warp_assets::background_directory();
            let path = directory.ok()?.join("background.png");
            load_background(&path).ok()
        })
        .clone()
}

pub fn apply_lite_background(theme: &mut WarpTheme) {
    theme.set_background_image(startup_background());
}

#[cfg(test)]
#[path = "backgrounds_tests.rs"]
mod tests;
