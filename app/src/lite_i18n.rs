pub(crate) const fn choose(english: &'static str, chinese: &'static str) -> &'static str {
    if cfg!(feature = "warp_lite") {
        chinese
    } else {
        english
    }
}

pub(crate) fn text(value: &str) -> &str {
    if !cfg!(feature = "warp_lite") {
        return value;
    }
    match value {
        "Cancel" => "取消",
        "Close" => "关闭",
        "Confirm" => "确认",
        "Save" => "保存",
        "Delete" => "删除",
        "Remove" => "移除",
        "Rename" => "重命名",
        "Copy" => "复制",
        "Paste" => "粘贴",
        "Cut" => "剪切",
        "Select All" => "全选",
        "Select all" => "全选",
        "Copy URL" => "复制链接地址",
        "Copy path" => "复制路径",
        "Copy link" => "复制链接",
        "Open link" => "打开链接",
        "Open in editor" => "在编辑器中打开",
        "Show in Finder" => "在 Finder 中显示",
        "Show containing folder" => "打开所在文件夹",
        "Copy command" => "复制命令",
        "Copy commands" => "复制命令",
        "Copy output" => "复制输出",
        "Copy filtered output" => "复制筛选后的输出",
        "Copy prompt" => "复制提示符",
        "Copy right prompt" => "复制右侧提示符",
        "Copy working directory" => "复制工作目录",
        "Copy git branch" => "复制 Git 分支",
        "Insert into input" => "插入输入框",
        "Find within block" => "在命令块中查找",
        "Find within blocks" => "在所选命令块中查找",
        "Scroll to top of block" => "滚动到命令块顶部",
        "Scroll to top of blocks" => "滚动到所选命令块顶部",
        "Scroll to bottom of block" => "滚动到命令块底部",
        "Scroll to bottom of blocks" => "滚动到所选命令块底部",
        "Toggle block filter" => "筛选命令块输出",
        "Toggle bookmark" => "添加或取消书签",
        "Clear Blocks" => "清空命令块",
        "Split pane right" => "向右分屏",
        "Split pane left" => "向左分屏",
        "Split pane down" => "向下分屏",
        "Split pane up" => "向上分屏",
        "Close pane" => "关闭分屏",
        "Edit prompt" => "编辑提示符",
        "Command search" => "搜索命令历史",
        "New worktree config" => "新建 worktree 配置",
        "New tab config" => "新建终端配置",
        "Previous session" => "上次会话",
        "Loading session..." => "正在加载会话…",
        "Loading..." => "正在加载…",
        "No results found." => "未找到结果。",
        "Filter block output" => "筛选命令块输出",
        "Bookmark this block to quickly scroll to it" => "为命令块添加书签，方便快速定位",
        "Search" => "搜索",
        "Search history" => "搜索命令历史",
        "Search actions" => "搜索操作",
        "Search sessions" => "搜索终端",
        "Search tabs" => "搜索标签页",
        "Search launch configurations" => "搜索终端配置",
        "Search commands" => "搜索命令",
        "Search blocks" => "搜索命令块",
        "history" => "历史",
        "actions" => "操作",
        "sessions" => "终端",
        "tabs" => "标签页",
        "launch configurations" => "终端配置",
        "commands" => "命令",
        "blocks" => "命令块",
        "Find" => "查找",
        "Default" => "默认",
        "None" => "无",
        "Always" => "始终",
        "Never" => "从不",
        "Settings" => "设置",
        "Terminal" => "终端",
        "Working Directory" | "Working directory" => "工作目录",
        "User" => "用户",
        "Host" => "主机",
        "Python Virtualenv" => "Python 虚拟环境",
        "Conda Environment" => "Conda 环境",
        "Node.js Version" => "Node.js 版本",
        "Date" => "日期",
        "Time (12-hour format)" => "时间（12 小时制）",
        "Time (24-hour format)" => "时间（24 小时制）",
        "Git Branch" => "Git 分支",
        "Git Branch Status" => "Git 分支状态",
        _ => value,
    }
}

pub(crate) fn copy_label(value: &str) -> String {
    if cfg!(feature = "warp_lite") {
        format!("复制{}", text(value))
    } else {
        format!("Copy {value}")
    }
}
