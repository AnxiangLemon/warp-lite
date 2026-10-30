use super::*;

#[cfg(feature = "warp_lite")]
#[test]
fn chinese_search_matches_terminal_settings() {
    for (terms, query) in [
        (
            "text terminal font family font size line height monospace",
            "字体 字号",
        ),
        ("starting working directory pwd session", "工作目录"),
        ("restore session window tab pane startup", "恢复 标签页"),
        ("max block size lines maximum limit memory", "历史 行数"),
        ("copy on select", "选中 复制"),
        ("startup shell session", "启动"),
        ("word smart select semantic separator", "智能 单词"),
        ("about warp version", "版本"),
        ("tab indicator", "标签页 状态标记"),
        ("jump to bottom of block button", "命令块 底部"),
    ] {
        let query = search_query(query);
        assert!(
            query.split_whitespace().all(|word| terms.contains(word)),
            "{query}"
        );
    }
    assert!(
        !search_query("项目浏览器")
            .split_whitespace()
            .all(|word| "font size".contains(word))
    );
}

#[cfg(feature = "warp_lite")]
#[test]
fn option_labels_change_without_translating_shells_or_paths() {
    assert_eq!(text("Default"), "默认");
    assert_eq!(text("Previous session's directory"), "沿用上一个终端的目录");
    assert_eq!(text("PowerShell"), "PowerShell");
    assert_eq!(label("JetBrains Mono (default)"), "JetBrains Mono（默认）");
    assert_eq!(text("D:\\My\\project"), "D:\\My\\project");
}

#[cfg(feature = "warp_lite")]
#[test]
fn shortcut_labels_use_stable_action_names() {
    assert_eq!(
        binding_label("input:search_command_history"),
        Some("搜索命令历史")
    );
    assert_eq!(binding_label("pane_group:add_right"), Some("向右分屏"));
    assert_eq!(binding_label("workspace:toggle_ai_assistant"), None);
}

#[cfg(not(feature = "warp_lite"))]
#[test]
fn full_build_retains_english_labels() {
    assert_eq!(text("Default"), "Default");
    assert_eq!(text("Font weight"), "Font weight");
    assert_eq!(
        label("JetBrains Mono (default)"),
        "JetBrains Mono (default)"
    );
}
