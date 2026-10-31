use crate::themes::theme::ThemeKind;

pub fn text(value: &str) -> &str {
    if !cfg!(feature = "warp_lite") {
        return value;
    }
    match value {
        "Settings" => "设置",
        "Command" => "操作",
        "Characters considered part of a word" => "视为单词组成部分的字符",
        "Keyboard shortcuts are not synced to the cloud" => "快捷键仅保存在本机",
        "Regular" => "常规",
        "Medium" => "中等",
        "Semi Bold" | "Semibold" => "半粗体",
        "Thin" => "细体",
        "Enabled" => "已启用",
        "Disabled" => "已禁用",
        "Appearance" => "外观",
        "Features" => "终端",
        "Keyboard shortcuts" | "Configure keyboard shortcuts" => "快捷键",
        "About" => "关于",
        "Search" => "搜索设置",
        "Search fonts" => "搜索字体",
        "No settings match your search." => "没有找到匹配的设置。",
        "Open settings file" => "打开设置文件",
        "Open file" => "打开文件",
        "Click to learn more in docs" => "查看使用说明",
        "Window positions won't be restored on Wayland. " => "Wayland 环境下无法恢复窗口位置。",
        "See docs." => "查看说明",
        "Your settings file contains an error." | "Your settings file contains errors." => {
            "设置文件有误。"
        }
        "Restore windows, tabs, and panes on startup" => "启动时恢复窗口、标签页和分屏",
        "Default shell for new sessions" => "新终端默认使用的 Shell",
        "Working directory for new sessions" => "新终端的工作目录",
        "Enable reopening of closed sessions" => "允许重新打开已关闭的终端",
        "Grace period (seconds)" => "可恢复时间（秒）",
        "Maximum rows in a block" => "每个命令块最多保留的行数",
        "Syntax highlighting for commands" => "命令语法高亮",
        "Open completions menu as you type" => "输入时显示补全菜单",
        "Tab key behavior" => "Tab 键的作用",
        "Ctrl+Tab behavior:" => "Ctrl+Tab 切换方式",
        "Double-click smart selection" => "双击智能选择文本",
        "Copy on select" => "选中文本时自动复制",
        "Middle-click to paste" => "鼠标中键粘贴",
        "Lines scrolled by mouse wheel interval" => "鼠标滚轮每次滚动的行数",
        "Allowed Values: 1-20" => "允许范围：1–20",
        "Supports floating point values between 1 and 20." => "支持 1 到 20 之间的小数。",
        "Use Audible Bell" => "启用终端提示音",
        "Sync with OS" => "跟随系统主题",
        "Light" => "浅色主题",
        "Dark" => "深色主题",
        "Current theme" => "当前主题",
        "Automatically switch between light and dark themes when your system does." => {
            "随系统自动切换浅色和深色主题。"
        }
        "Terminal font" => "终端字体",
        "Font size (px)" => "字号（像素）",
        "Font weight" => "字重",
        "Line height" => "行高",
        "Reset to default" => "恢复默认值",
        "View all available system fonts" => "显示所有系统字体",
        "Input position" => "命令输入框位置",
        "Show tab indicators" => "显示标签页状态标记",
        "Tab close button position" => "标签页关闭按钮的位置",
        "Preserve active tab color for new tabs" => "新标签页沿用当前标签页的颜色",
        "Use vertical tab layout" => "使用纵向标签栏",
        "Show vertical tabs panel in restored windows" => "恢复窗口时显示纵向标签栏",
        "When enabled, reopening or restoring a window opens the vertical tabs panel even if it was closed when the window was last saved." => {
            "重新打开窗口时，自动展开纵向标签栏。"
        }
        "Dim inactive panes" => "调暗未激活的分屏",
        "Focus follows mouse" => "鼠标悬停时切换分屏焦点",
        "Compact mode" => "紧凑显示",
        "Show block dividers" => "显示命令块分隔线",
        "Show Jump to Bottom of Block button" => "显示跳转到命令块底部的按钮",
        "Cursor type" => "光标形状",
        "Cursor type is disabled in Vim mode" => "Vim 模式下无法调整光标形状",
        "Blinking cursor" => "光标闪烁",
        "Default" => "默认",
        "Preferred graphics backend" => "首选渲染后端",
        "Current backend" => "当前渲染后端",
        "Changes will apply to new windows." => "更改将在新窗口中生效。",
        "Custom" => "自定义",
        "Advanced" => "分别设置",
        "Home directory" => "用户主目录",
        "Previous directory" | "Previous session's directory" => "沿用上一个终端的目录",
        "Custom directory" => "指定目录",
        "New window" => "新窗口",
        "New tab" => "新标签页",
        "Split pane" | "Split Pane" => "新分屏",
        "Directory path" => "输入目录路径",
        "Executable path" => "输入 Shell 可执行文件路径",
        "Pin to the bottom (Warp mode)" => "固定在底部",
        "Pin to the top (Reverse mode)" => "固定在顶部",
        "Start at the top (Classic mode)" => "从顶部开始，随输出向下移动",
        "Activate previous/next tab" => "按标签页顺序切换",
        "Cycle most recent session" => "切换最近使用的终端或分屏",
        "Cycle most recent tab" => "切换最近使用的标签页",
        "Completions" | "Open completions menu" => "打开补全菜单",
        "Autosuggestions" | "Accept autosuggestion" => "接受历史建议",
        "User defined" | "User Defined" => "自定义快捷键",
        "Left" => "左侧",
        "Right" => "右侧",
        "Always" => "始终",
        "Never" => "从不",
        "None" => "无",
        "Normal" | "normal" => "常规",
        "Bold" | "bold" => "粗体",
        "Block" | "block" => "方块",
        "Bar" | "bar" | "Beam" | "beam" => "竖线",
        "Underline" | "underline" => "下划线",
        "Search by name or by keys (ex. \"cmd d\")" => "按功能或按键搜索（例如 Ctrl Tab）",
        "This shortcut conflicts with other keybinds" => "此快捷键与其他操作冲突",
        "Press new keyboard shortcut" => "请按下新的快捷键",
        "Cancel" => "取消",
        "Clear" => "清除",
        "Save" => "保存",
        "Add your own custom keybindings to existing actions below." => {
            "点击下方操作，修改对应的快捷键。"
        }
        "Use" => "使用",
        "to reference these keybindings in a side pane at anytime." => "可随时在侧栏查看快捷键。",
        "Copyright 2026 Warp" => "版权所有 © 2026 Warp",
        "Split pane right" => "向右分屏",
        "Split pane left" => "向左分屏",
        "Split pane down" => "向下分屏",
        "Split pane up" => "向上分屏",
        "Close pane" => "关闭分屏",
        _ => crate::lite_i18n::text(value),
    }
}

#[cfg(test)]
#[path = "lite_i18n_tests.rs"]
mod tests;

pub fn search_query(query: &str) -> String {
    if !cfg!(feature = "warp_lite") {
        return query.to_string();
    }
    let mut query = query.to_lowercase();
    for (chinese, english) in [
        ("快捷键", " keyboard "),
        ("启动", " startup "),
        ("会话", " session "),
        ("终端", " terminal "),
        ("单词", " word "),
        ("字符", " word "),
        ("智能", " smart "),
        ("版本", " version "),
        ("状态标记", " indicator "),
        ("标记", " indicator "),
        ("底部", " bottom "),
        ("顶部", " top "),
        ("字体", " font "),
        ("字号", " font size "),
        ("字重", " font weight "),
        ("行高", " line height "),
        ("主题", " theme "),
        ("颜色", " color "),
        ("输入位置", " input position "),
        ("输入框", " input "),
        ("默认", " "),
        ("工作目录", " working directory "),
        ("目录", " directory "),
        ("恢复", " restore "),
        ("重开", " reopen "),
        ("关闭", " close "),
        ("历史", " block "),
        ("命令块", " block "),
        ("保留", " limit "),
        ("行数", " lines "),
        ("滚动", " scroll "),
        ("滚轮", " wheel "),
        ("复制", " copy "),
        ("选择", " select "),
        ("选中", " select "),
        ("粘贴", " paste "),
        ("中键", " middle "),
        ("双击", " smart "),
        ("标签页", " tab "),
        ("标签", " tab "),
        ("纵向", " vertical "),
        ("侧栏", " sidebar "),
        ("分屏", " pane "),
        ("窗口", " window "),
        ("提示音", " audible bell "),
        ("语法高亮", " syntax highlighting "),
        ("补全", " completions "),
        ("切换", " switch "),
        ("紧凑", " compact "),
        ("分隔线", " dividers "),
        ("调暗", " dim "),
        ("悬停", " focus follows mouse "),
        ("光标", " cursor "),
        ("闪烁", " blinking "),
        ("关于", " about "),
    ] {
        query = query.replace(chinese, english);
    }
    query
}

pub fn label(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    let translated = text(value);
    if cfg!(feature = "warp_lite") {
        translated.replace(" (default)", "（默认）")
    } else {
        translated.to_owned()
    }
}

pub fn theme_label(kind: &ThemeKind) -> String {
    if !cfg!(feature = "warp_lite") {
        return kind.to_string();
    }
    match kind {
        ThemeKind::Light => "浅色主题",
        ThemeKind::Dark => "深色主题",
        ThemeKind::Adeberry => "紫莓",
        ThemeKind::Phenomenon => "幻象",
        ThemeKind::Dracula => "德古拉",
        ThemeKind::FancyDracula => "绚彩德古拉",
        ThemeKind::CyberWave => "赛博浪潮",
        ThemeKind::SolarFlare => "太阳耀斑",
        ThemeKind::SolarizedDark => "Solarized 深色",
        ThemeKind::SolarizedLight => "Solarized 浅色",
        ThemeKind::WillowDream => "柳色梦境",
        ThemeKind::DarkCity => "暗夜都市",
        ThemeKind::GruvboxDark => "Gruvbox 深色",
        ThemeKind::GruvboxLight => "Gruvbox 浅色",
        ThemeKind::RedRock => "赤岩",
        ThemeKind::JellyFish => "水母",
        ThemeKind::Leafy => "绿叶",
        ThemeKind::Koi => "锦鲤",
        ThemeKind::Snowy => "初雪",
        ThemeKind::PinkCity => "粉色都市",
        ThemeKind::Marble => "大理石",
        ThemeKind::SentReferralReward
        | ThemeKind::ReceivedReferralReward
        | ThemeKind::Custom(_)
        | ThemeKind::CustomBase16(_)
        | ThemeKind::InMemory(_) => return kind.to_string(),
    }
    .to_owned()
}

pub fn binding_label(name: &str) -> Option<&'static str> {
    Some(match name {
        "root_view:new_window" | "workspace:new_window" => "新建窗口",
        "workspace:new_tab" => "新建标签页",
        "workspace:close_window" => "关闭窗口",
        "workspace:close_active_tab" => "关闭当前标签页",
        "workspace:close_other_tabs" => "关闭其他标签页",
        "workspace:close_tabs_right_active_tab" => "关闭右侧标签页",
        "workspace:rename_active_tab" => "重命名当前标签页",
        "workspace:rename_active_pane" => "重命名当前分屏",
        "workspace:new_tab_group" => "新建标签页分组",
        "workspace:new_tab_group_from_active_or_selected_tabs" => "将所选标签页加入新分组",
        "workspace:remove_active_or_selected_tabs_from_group" => "将标签页移出分组",
        "workspace:pin_active_tab" => "固定当前标签页",
        "workspace:unpin_active_tab" => "取消固定当前标签页",
        "workspace:move_tab_left" => "向左移动标签页",
        "workspace:move_tab_right" => "向右移动标签页",
        "workspace:activate_next_tab" => "切换到下一个标签页",
        "workspace:activate_prev_tab" => "切换到上一个标签页",
        "workspace:activate_first_tab" => "切换到第一个标签页",
        "workspace:activate_second_tab" => "切换到第二个标签页",
        "workspace:activate_third_tab" => "切换到第三个标签页",
        "workspace:activate_fourth_tab" => "切换到第四个标签页",
        "workspace:activate_fifth_tab" => "切换到第五个标签页",
        "workspace:activate_sixth_tab" => "切换到第六个标签页",
        "workspace:activate_seventh_tab" => "切换到第七个标签页",
        "workspace:activate_eighth_tab" => "切换到第八个标签页",
        "workspace:activate_last_tab" => "切换到最后一个标签页",
        "workspace:reopen_closed_session" | "workspace:undo_close" => "重新打开已关闭的终端",
        "workspace:show_settings" => "打开设置",
        "workspace:show_settings_appearance_page" => "打开外观设置",
        "workspace:show_settings_features_page" => "打开终端设置",
        "workspace:show_settings_keyboard_shortcuts_page"
        | "workspace:show_keybinding_settings"
        | "workspace:toggle_keybindings_page" => "打开快捷键设置",
        "workspace:show_settings_about_page" => "关于 Warp Lite",
        "workspace:show_theme_chooser" => "选择主题",
        "workspace:open_settings_file" => "打开设置文件",
        "workspace:toggle_command_palette" => "搜索操作",
        "workspace:toggle_navigation_palette" => "切换终端",
        "workspace:increase_font_size" => "增大字号",
        "workspace:decrease_font_size" => "减小字号",
        "workspace:reset_font_size" => "恢复默认字号",
        "workspace:increase_zoom" => "放大界面",
        "workspace:decrease_zoom" => "缩小界面",
        "workspace:reset_zoom" => "恢复默认缩放",
        "workspace:copy_current_path" => "复制当前目录",
        "workspace:shift_focus_left" => "将焦点移到左侧",
        "workspace:shift_focus_right" => "将焦点移到右侧",
        "workspace:terminate_app" => "退出 Warp Lite",
        "pane_group:add_right" => "向右分屏",
        "pane_group:add_left" => "向左分屏",
        "pane_group:add_down" => "向下分屏",
        "pane_group:add_up" => "向上分屏",
        "pane_group:toggle_maximize_pane" => "最大化或还原当前分屏",
        "pane_group:navigate_next" => "切换到下一个分屏",
        "pane_group:navigate_prev" => "切换到上一个分屏",
        "terminal:copy" | "editor_view:copy" => "复制",
        "terminal:paste" | "editor_view:paste" => "粘贴",
        "terminal:copy_commands" => "复制命令",
        "terminal:copy_outputs" => "复制命令输出",
        "terminal:find" => "查找终端内容",
        "terminal:clear_blocks" | "input:clear_screen" => "清空终端内容",
        "terminal:bookmark_selected_block" => "标记当前命令块",
        "terminal:scroll_up_one_page" => "向上滚动一页",
        "terminal:scroll_down_one_page" => "向下滚动一页",
        "terminal:scroll_to_top_of_selected_block" => "跳到命令块顶部",
        "terminal:scroll_to_bottom_of_selected_block" => "跳到命令块底部",
        "input:search_command_history" | "workspace:show_command_search" => "搜索命令历史",
        "input:open_completion_suggestions" => "打开补全菜单",
        "input:insert_autosuggestion" => "接受历史建议",
        "editor_view:select_all" => "全选",
        "editor_view:cut" => "剪切",
        "editor_view:undo" => "撤销",
        "editor_view:redo" => "重做",
        "editor_view:move_to_line_start" => "移到行首",
        "editor_view:move_to_line_end" => "移到行尾",
        _ => return None,
    })
}
