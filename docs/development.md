# Warp Lite 开发说明

## 源码结构

| 路径 | 用途 |
| --- | --- |
| `app/src/bin/lite.rs` | Lite 入口、独立应用标识与本地服务配置 |
| `app/src/lite_cli.rs` | 本地终端 CLI 参数，不接受云端操作与凭据参数 |
| `app/Cargo.toml` | `warp_lite`、`agent_runtime`、`cloud_services` 等 feature 边界 |
| `app/src/terminal/`、`app/src/pane_group/`、`app/src/workspace/` | 终端、分屏、标签分组及恢复 |
| `app/src/settings_view/lite_i18n.rs` | 中文设置文案与搜索词映射 |
| `crates/http_client/`、`crates/warp_server_client/` | 显式区分离线接口与网络 transport |
| `crates/managed_secrets/`、`crates/mcp/` | 禁用云凭据和 MCP transport 时保留所需接口 |
| `crates/warp_assets/` | Lite 压缩资源与资源过滤 |
| `crates/integration/src/test/lite.rs` | 隔离的 Windows GUI 终端与恢复测试 |
| `script/run-lite.ps1`、`script/package-lite.ps1` | 运行与打包 |
| `script/verify-lite-dependencies.ps1` | 检查 Lite 运行依赖中是否混入已移除的服务组件 |

当前使用共享模块内的 feature 门控，移除了之前不完整的 `ai_lite/`、`ai_assistant_lite/` 等替代模块。持久化类型和终端共用接口继续保留，避免精简过程中破坏历史数据与会话恢复。

## 构建与检查

```powershell
cargo check -p warp --bin warp-lite --no-default-features --features warp_lite --locked
cargo build -p warp --bin warp-lite --profile lite --no-default-features --features warp_lite --locked
.\script\verify-lite-dependencies.ps1
```

Windows 的运行内存可用以下命令采样：

```powershell
.\script\measure-lite-memory.ps1 -ProcessId <主进程 PID> -ReportPath target/memory.json
```

对比时使用相同构建类型、窗口尺寸、终端数量和页面，分别记录私有工作集、总工作集、
私有提交量；EXE 文件大小不是运行内存。第二个带
`--parent-handle` 的小进程负责崩溃恢复，测量脚本不把它或 shell 计入主进程。

Windows Lite 使用 `wgpu::MemoryHints::MemoryUsage`，让图形内存池从较小块开始按需增长，
并将 DirectX 的 `max_non_sampler_bindings` 从默认 1,000,000 降为 65,536，避免在设备创建时
预留过大的描述符堆。它限制同时存活的 GPU 资源绑定数量，不是终端历史行数。
完整版保留默认策略。Lite 不自动扫描其他终端的配置，字体选择器只枚举名称与类型，
在选中字体时加载实际字体数据。

Windows Lite 默认先初始化 DirectX；只有初始化失败、显卡属于已知不稳定型号、用户指定
其他后端或显式查询完整后端列表时，才创建包含其余后端的实例。`WGPU_BACKEND` 环境变量
仍优先生效，Vulkan / OpenGL 回退能力保留。Lite 未显示的图形后端设置不触发后台枚举。

字体列表在首次展开选择器时异步枚举；普通设置页面不扫描全部系统字体。SVG 的字体数据库
在解析实际文本节点时才初始化，纯路径图标不加载字体；包含文本的 SVG 保留系统字体及
内置 Roboto 回退。图形内存池、字体和图片缓存随实际内容增长，不通过强制清空工作集降低读数。

`lite` profile 对主应用使用单个 codegen unit，配合既有 Thin LTO 和符号剥离缩减 EXE。
这会增加优化构建时间；日常开发继续使用 Debug 构建，终端核心保持 `opt-level = 3`。

Lite 背景按 `background.png`、`background.jpg`、`background.jpeg` 顺序查找第一张有效图片。
文件读取、格式检测、解码、EXIF 旋转和缩放均在后台执行；首屏先使用主题底色，加载完成后
更新当前主题。图片快照归 AppearanceManager 所有，同一次运行中切换主题不重新读取文件。
恢复终端时，目录存在性检查在 shell 初始化的后台任务执行，避免 WSL / UNC 目录唤醒阻塞
界面。WSL 主目录查询异步执行，Windows 启动器使用本地目录，Linux 工作目录仍通过
`WARP_INITIAL_WORKING_DIR` 传给 shell。

排查启动可临时设置 `RUST_LOG=info,warp=debug`，本地日志中的 `Lite startup timings:`
记录初始化至实际首帧绘制的分阶段毫秒数。`Shell is bootstrapped` 是后续 shell 就绪里程碑，
不能将窗口出现时间当作 shell 就绪时间。性能对照须固定构建类型、背景、会话和配置，避开
并行编译；`WARP_DATA_PROFILE` 仅能隔离 Debug 构建，不能用于隔离发布版。

格式和完整 workspace 的 Clippy 命令以 `script/presubmit` 为准，使用 Git Bash：

```bash
./script/format
./script/format --check
cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings
cargo clippy -p warp --all-targets --tests -- -D warnings
cargo clippy -p warp_completer --all-targets --tests -- -D warnings
```

完整 workspace 的检查还涉及上游保留的 JS completer，需要 Node、Corepack / Yarn 和对应 JS 依赖。Lite 的构建不需要这条 JS completer 路径。Windows 构建 helper 在 Cargo job 内运行，不应请求 job breakaway。

## 针对性单元测试

有 `cargo-nextest` 时优先用 nextest；也可执行：

```powershell
cargo test -p http_client -p warp_server_client -p warp_managed_secrets -p mcp --lib --no-default-features --locked
cargo test -p warp --lib --no-default-features --features warp_lite --locked lite_
cargo test -p warp --lib --no-default-features --features warp_lite --locked terminal::session_types::tests
cargo test -p warp --lib --no-default-features --features warp_lite --locked themes::backgrounds::tests
cargo test -p warp --lib --no-default-features --features warp_lite --locked server::telemetry::offline_tests
cargo test -p warp --lib --no-default-features --features warp_lite --locked test_sqlite_round_trips_vertical_tabs_panel_open
cargo test -p warpui_core --lib --locked image_cache::tests
cargo clippy -p warpui -p warpui_core --lib --locked -- -D warnings
```

## 实际 GUI 验证

```powershell
cargo build -p integration --bin integration --no-default-features --features warp_lite --locked
$env:WARPUI_USE_REAL_DISPLAY_IN_INTEGRATION_TESTS = '1'
$env:WARP_INTEGRATION_TEST_ARTIFACTS_DIR = Join-Path $PWD 'target/lite-test-artifacts'
.\target\debug\integration.exe test_lite_terminal_smoke
.\target\debug\integration.exe test_lite_terminal_management
.\target\debug\integration.exe test_lite_chinese_settings
.\target\debug\integration.exe test_lite_restore_named_tabs
.\target\debug\integration.exe test_lite_close_confirmation
.\target\debug\integration.exe test_lite_jpeg_background
.\target\debug\integration.exe test_lite_fixed_background

# 检查延迟初始化后仍可显式使用 Vulkan。
$env:WGPU_BACKEND = 'vulkan'
.\target\debug\integration.exe test_lite_terminal_smoke
Remove-Item Env:WGPU_BACKEND
```

测试使用独立的 `WarpIntegrationLiteSmoke-*` 应用标识与数据目录，不使用日常 Warp Lite 数据库。实际显示测试逐个运行，避免同时争用窗口和焦点。

OpenGL 可用 `WGPU_BACKEND=gl` 单独验证。当前原生 OpenGL surface 不提供 `COPY_SRC`，
所以内置的帧读取截图步骤会失败；应使用桌面截图核对画面，不能把功能断言通过等同于整项
截图集成测试通过。

跨进程恢复使用 `test_lite_restore_saved_tabs`。先从 smoke test 输出的 evidence JSON 获取数据库与项目目录，将 SQLite 一致性备份路径设置为 `WARP_LITE_RESTORE_DATABASE`、项目目录设置为 `WARP_LITE_RESTORE_DIRECTORY`，再执行该测试；设置 `WARP_LITE_EXPECT_RESTORED_GROUPS=1` 还会检查分组和侧栏恢复。SQLite 使用 WAL，不能仅复制仍有 WAL 数据的主数据库文件；应使用 SQLite backup API，或在确认进程退出并 checkpoint 后复制。

## 资源与提交

保留 `Cargo.lock`、数据库 migrations、ConPTY / OpenConsole、字体、shell bootstrap 与许可证。不要用全局 `*.exe`、`*.dll` 或 `*.txt` 规则排除运行资源。

`target*/`、`dist/`、`.note/`、排错日志和一次性 import 改写脚本不提交。提交前检查 `git diff --check` 和暂存区；不将旧分发包作为当前源码的构建证明。

本仓库默认远端 `origin` 指向 `AnxiangLemon/warp-lite`，不配置自动合并上游。GitHub Actions 的范围见 [.github/workflows/README.md](../.github/workflows/README.md)。
