use std::path::PathBuf;

use super::*;
use crate::themes::theme::CustomTheme;
use crate::user_config;

fn custom(path: PathBuf) -> ThemeKind {
    ThemeKind::Custom(CustomTheme::new("Custom".to_string(), path))
}

fn custom_base16(path: PathBuf) -> ThemeKind {
    ThemeKind::CustomBase16(CustomTheme::new("Base16 Custom".to_string(), path))
}

#[test]
fn theme_kind_syncs_custom_theme_under_theme_root() {
    let setting = Theme::new(Some(custom(user_config::themes_dir().join("custom.yml"))));

    assert!(setting.current_value_is_syncable());
}

#[test]
fn theme_kind_does_not_sync_custom_theme_outside_theme_root() {
    let setting = Theme::new(Some(custom(std::env::temp_dir().join("custom.yml"))));

    assert!(!setting.current_value_is_syncable());
}

#[test]
fn theme_kind_syncs_custom_base16_theme_under_theme_root() {
    let setting = Theme::new(Some(custom_base16(
        user_config::themes_dir().join("base16/custom.yml"),
    )));

    assert!(setting.current_value_is_syncable());
}

#[test]
fn selected_system_themes_sync_when_custom_paths_are_under_theme_root() {
    let setting = SystemThemes::new(Some(SelectedSystemThemes {
        light: custom(user_config::themes_dir().join("light.yml")),
        dark: custom_base16(user_config::themes_dir().join("dark.yml")),
    }));

    assert!(setting.current_value_is_syncable());
}

#[test]
fn selected_system_themes_do_not_sync_when_any_custom_path_is_outside_theme_root() {
    let setting = SystemThemes::new(Some(SelectedSystemThemes {
        light: custom(user_config::themes_dir().join("light.yml")),
        dark: custom(std::env::temp_dir().join("dark.yml")),
    }));

    assert!(!setting.current_value_is_syncable());
}

#[test]
fn built_in_theme_settings_remain_syncable() {
    let theme = Theme::new(Some(ThemeKind::Dark));
    let system_themes = SystemThemes::new(Some(SelectedSystemThemes {
        light: ThemeKind::Light,
        dark: ThemeKind::Dark,
    }));

    assert!(theme.current_value_is_syncable());
    assert!(system_themes.current_value_is_syncable());
}

#[test]
#[cfg(feature = "warp_lite")]
fn removed_lite_background_native_preference_does_not_break_theme_initialization() {
    warpui::App::test((), |mut app| async move {
        let _guard = warp_core::features::FeatureFlag::SettingsFile.override_enabled(false);
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("user_preferences.json");
        let old_background = r#"{"File":{"path":"C:/missing/background.jpg"}}"#;
        std::fs::write(
            &path,
            serde_json::to_vec(&serde_json::json!({
                "prefs": {
                    "LiteBackground": old_background,
                    "SystemTheme": "true",
                    "Theme": serde_json::to_string(&ThemeKind::Dark).unwrap(),
                }
            }))
            .unwrap(),
        )
        .unwrap();
        let preferences =
            warpui_extras::user_preferences::file_backed::FileBackedUserPreferences::new(path)
                .unwrap();
        app.add_singleton_model(move |_| settings::PrivatePreferences::new(Box::new(preferences)));
        app.add_singleton_model(|_| settings::manager::SettingsManager::default());
        ThemeSettings::register(&mut app);
        app.read(|ctx| {
            use warpui::SingletonEntity;

            let theme = ThemeSettings::as_ref(ctx);
            assert!(*theme.use_system_theme);
            assert_eq!(*theme.theme_kind, ThemeKind::Dark);
            assert_eq!(
                settings::PrivatePreferences::as_ref(ctx)
                    .read_value("LiteBackground")
                    .unwrap()
                    .as_deref(),
                Some(old_background)
            );
            assert!(
                !settings::manager::SettingsManager::as_ref(ctx)
                    .public_storage_keys()
                    .any(|key| key == "LiteBackground")
            );
        });
    });
}

#[test]
#[cfg(feature = "warp_lite")]
fn removed_lite_background_toml_setting_does_not_break_theme_initialization() {
    warpui::App::test((), |mut app| async move {
        let _guard = warp_core::features::FeatureFlag::SettingsFile.override_enabled(true);
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("settings.toml");
        std::fs::write(
            &path,
            r#"
lite_background = { File = { path = "C:/missing/background.jpg" } }

[appearance.themes]
system_theme = true

[appearance.themes.lite_background]
kind = "File"
path = "C:/missing/another-background.jpg"
"#,
        )
        .unwrap();
        let (preferences, parse_error) =
            warpui_extras::user_preferences::toml_backed::TomlBackedUserPreferences::new(
                path.clone(),
            );
        assert!(parse_error.is_none());
        app.add_singleton_model(move |_| settings::PublicPreferences::new(Box::new(preferences)));
        app.add_singleton_model(|_| settings::manager::SettingsManager::default());
        ThemeSettings::register(&mut app);
        app.read(|ctx| {
            use warpui::SingletonEntity;

            assert!(*ThemeSettings::as_ref(ctx).use_system_theme);
            assert!(
                !settings::manager::SettingsManager::as_ref(ctx)
                    .public_storage_keys()
                    .any(|key| key == "LiteBackground")
            );
        });
        assert!(
            std::fs::read_to_string(path)
                .unwrap()
                .contains("another-background.jpg")
        );
    });
}
