use warpui_core::AssetProvider;

use super::Assets;

#[test]
fn terminal_font_preserves_original_bytes() {
    let font = AssetProvider::get(&Assets, "bundled/fonts/hack/Hack-Regular.ttf").unwrap();

    assert_eq!(
        &*font,
        include_bytes!("../../../app/assets/bundled/fonts/hack/Hack-Regular.ttf")
    );
}

#[test]
fn terminal_icon_preserves_original_bytes() {
    let icon = AssetProvider::get(&Assets, "bundled/svg/terminal.svg").unwrap();

    assert_eq!(
        &*icon,
        include_bytes!("../../../app/assets/bundled/svg/terminal.svg")
    );
}

#[test]
fn missing_asset_returns_error() {
    assert!(AssetProvider::get(&Assets, "bundled/missing-file").is_err());
}

#[cfg(all(feature = "lite", not(target_family = "wasm")))]
#[test]
fn builtin_backgrounds_are_not_embedded() {
    assert!(<Assets as super::RustEmbed>::get("async/jpg/koi_bg.jpg").is_none());
    assert!(<Assets as super::RustEmbed>::get("async/jpg/phenomenon_bg.jpg").is_none());
}

#[cfg(all(feature = "lite", target_os = "windows"))]
#[test]
fn background_paths_follow_the_executable_directory() {
    use std::path::{Path, PathBuf};
    let executable = Path::new("C:/Program Files/中文终端/warp-lite.exe");
    assert_eq!(
        super::background_directory_for_executable(executable).unwrap(),
        PathBuf::from("C:/Program Files/中文终端/backgrounds")
    );
    assert_eq!(
        super::external_background_path(executable, "async/jpg/koi_bg.jpg"),
        Some(PathBuf::from(
            "C:/Program Files/中文终端/backgrounds/koi_bg.jpg"
        ))
    );
    assert!(super::background_directory_for_executable(Path::new("warp-lite.exe")).is_err());
}

#[cfg(all(feature = "lite", target_os = "windows"))]
#[test]
fn external_background_assets_reject_nested_and_absolute_paths() {
    use std::path::Path;
    let executable = Path::new("C:/Warp/warp-lite.exe");
    assert_eq!(
        super::external_background_path(executable, "async/jpg/../koi_bg.jpg"),
        None
    );
    assert_eq!(
        super::external_background_path(executable, "async/jpg/..\\koi_bg.jpg"),
        None
    );
    assert_eq!(
        super::external_background_path(executable, "async/jpg/C:other.jpg"),
        None
    );
    assert_eq!(
        super::external_background_path(executable, "async/jpg/"),
        None
    );
}
