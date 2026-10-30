#![cfg_attr(windows, windows_subsystem = "windows")]

use anyhow::Result;
use warp_core::AppId;
use warp_core::channel::{Channel, ChannelConfig, ChannelState, OzConfig, WarpServerConfig};

/// Local-only terminal build. Uses the real Warp terminal stack but points every
/// server-backed subsystem at loopback, so no login, cloud, or telemetry is used.
fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--version") {
        println!("Warp Lite 0.1.0");
        return Ok(());
    }

    let state = ChannelState::new(
        Channel::Oss,
        ChannelConfig {
            app_id: AppId::new("dev", "warp", "WarpLite"),
            logfile_name: "warp-lite.log".into(),
            server_config: WarpServerConfig::local_only(),
            oz_config: OzConfig::local_only(),
            telemetry_config: None,
            crash_reporting_config: None,
            autoupdate_config: None,
            mcp_static_config: None,
        },
    );
    ChannelState::set(state);

    warp::run()
}

#[cfg(all(not(feature = "extern_plist"), target_os = "macos"))]
embed_plist::embed_info_plist_bytes!(r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
    <key>CFBundleDevelopmentRegion</key><string>English</string>
    <key>CFBundleDisplayName</key><string>Warp Lite</string>
    <key>CFBundleExecutable</key><string>warp-lite</string>
    <key>CFBundleIdentifier</key><string>dev.warp.WarpLite</string>
    <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
    <key>CFBundleName</key><string>Warp Lite</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>0.1.0</string>
    <key>LSApplicationCategoryType</key><string>public.app-category.developer-tools</string>
    <key>NSHighResolutionCapable</key><true/>
    </dict>
    </plist>
"#.as_bytes());
