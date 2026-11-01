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
仍优先生效，Vulkan / OpenGL 回退能力保留。所有 Windows 环境使用同一个 Warp Lite 构建，
不区分服务器版与桌面版。“设置 → 外观 → 图形渲染”包含三个独立选项：

- 渲染设备：自动（GPU 优先，没有可用 GPU 时回退到 CPU）或 CPU 软件渲染。
- 图形后端：默认、DirectX 12、Vulkan、OpenGL；不可用的首选接口自动回退。
  DirectX 12 支持 Windows 内置的软件渲染设备；Vulkan / OpenGL 需要兼容的驱动，
  选择接口本身不代表使用了 GPU，也不保证更省 CPU。设备、后端修改后新建窗口或重启生效。
- 帧率上限：自动、15、30、60 FPS 或不限，修改立即生效。自动模式在 CPU 设备上最高
  30 FPS，并按实际绘制耗时留出恢复时间；硬件设备不增加应用层限帧。显式上限不额外
  自适应降频；不限仍受绘制耗时和显示同步约束。所有选项都按需绘制，不产生固定刷新定时器。

对应 TOML 设置为 `system.rendering_device`（`automatic` / `software`）、
`system.frame_rate_limit`（`automatic` / `fps_15` / `fps_30` / `fps_60` / `unlimited`）和
`system.preferred_graphics_backend`（如 `dx_12`）；以设置 UI 写出的值为准。
设置页显示实际设备类型和后端，不为填充下拉列表初始化其他图形驱动。

限帧在布局、surface 重配置与绘制之前合并刷新请求，保留最后一次重绘；最小化或隐藏的
窗口不绘制。软件渲染的交换链使用 VSync 并限制排队帧数；硬件设备保留原有提交模式。
CPU 软件渲染会缓存背景在当前窗口尺寸下的绘制结果，重复刷新时复用像素；最底层背景在
surface 支持时直接复制，否则用逐像素读取合成。更换图片、调整尺寸、缩放、透明度、圆角
或裁剪区域会重建缓存；每个窗口只保留一份，上限 64 MiB，超限使用原绘制路径。硬件设备
保持原路径。缓存不改变背景图片清晰度，也不新增固定刷新定时器；FPS 仍由上述设置控制。
可在有 GPU 的开发机上用以下环境变量复现软件渲染路径：

```powershell
$env:WARP_FORCE_SOFTWARE_RENDERING = '1'
$env:WGPU_BACKEND = 'dx12'
```

启动日志应显示 `Using Dx12 Cpu (Microsoft Basic Render Driver)`。验证完成后清除这两个
环境变量再启动，以恢复自动选择。性能对照须使用相同构建、窗口尺寸、缩放、内容与交互；
开发机的软件渲染结果不能替代 Windows Server / RDP 上的实测。
集成测试构建可设置 `WARP_TEST_DISABLE_BACKGROUND_RASTER_CACHE=1` 关闭背景栅格缓存，
与同一个二进制默认开启时对照；正式构建忽略该变量。背景测试包含连续缩放与固定尺寸重绘，
输出帧数、墙钟时间和进程累计 CPU 毫秒数；累计 CPU 时间不是任务管理器 CPU 百分比。

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

Windows 的 ConPTY 创建与进程启动在后台并行执行，恢复多个标签页不会在界面线程上逐个
等待。PowerShell 首次握手只枚举已加载的命令，完整命令发现继续在后台执行；后台命令的
runspace pool 在首次使用时创建，从各一个 runspace 开始增长，保留原有并发上限。

排查启动可临时设置 `RUST_LOG=info,warp::startup=debug,warpui::startup=debug`。
`Lite startup timings:` 记录初始化至实际首帧绘制的分阶段毫秒数，附加日志区分 GPU、
pipeline、shell 路径解析、PTY 创建和 `shell_ready`。最后一项是 shell integration 的
Bootstrapped 事件，不是窗口出现时间，也不是首条用户命令的执行耗时。

Lite 启用 `warp_core/data_profiles`，因此本次起的优化构建也支持 `WARP_DATA_PROFILE`。
名称必须为 1–64 个 ASCII 字母、数字、下划线或连字符；未设置时仍使用正常数据目录。
旧发布版不支持该隔离方式，不能直接套用到下面的测量脚本。

`script/measure-lite-startup.ps1` 对两份支持 profile 隔离的完整包做交替测量。种子目录需
包含安全快照 `warp.sqlite` 和 `settings.toml`；脚本为每次启动复制独立 profile，保留
原数据，先预热一对，再默认测量五对。两包须使用相同 DLL、背景、会话和设置，并在编译
结束后运行。脚本不关闭 WSL 或清空系统缓存，结果属于重复热启动。示例：

本机清理后的基线包和会话种子归档在 `.note/startup-cleanup-2026-10-31/`（不提交）。
重跑本机对照前，可将 `startup-baseline-package.zip` 解压至
`target/lite-release-startup-before-package/`，并将归档目录中的 `startup-evidence/lite-startup-seed/`
复制到 `target/lite-startup-seed/`。其他开发机需准备自己的基线包和安全会话快照。

```powershell
./script/measure-lite-startup.ps1 `
  -BeforeExecutable 'target/lite-release-startup-before-package/warp-lite.exe' `
  -AfterExecutable 'dist/Warp Lite Fast Startup/warp-lite.exe' `
  -SeedDirectory 'target/lite-startup-seed' `
  -ExpectedShells 5 -Pairs 5 -OutputPath 'target/lite-release-startup-comparison.json'
```

要测量 PowerShell 的首次提示符，先用 `script/prepare-lite-startup-probe.py SOURCE DEST`
为两包各制作一个独立副本，再把副本路径传给测量脚本，并加
`-ExpectedPowerShellPrompts 3`（改为快照中实际 PowerShell 数量）。探针在首次
post-bootstrap Precmd 发出后记录跨进程 Stopwatch 时间，属于提示符准备完成的里程碑；
仍不等同于首条用户命令的往返时间。探针副本仅供测量，不应作为正常使用的包分发。

PowerShell bootstrap 的批量输出、错误帧、取消和按需 pool 生命周期可单独验证：

```powershell
python script/test-pwsh-bootstrap.py `
  --shell "$env:SystemRoot/System32/WindowsPowerShell/v1.0/powershell.exe" `
  --shell "$env:LOCALAPPDATA/Microsoft/WindowsApps/pwsh.exe"
```

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
.\target\debug\integration.exe test_lite_software_rendering
.\target\debug\integration.exe test_lite_software_rendering_background
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

## 多终端资源与故障隔离验证

资源数据应使用优化构建；测试期间不要同时编译或运行另一组 GUI 测试。下面的测试构建只为
`warp` crate 增加代码生成并行度，仍使用 `lite` 优化与 Thin LTO；分发构建保持原配置。

```powershell
cargo build -p integration --bin integration --profile lite --no-default-features --features warp_lite --locked --target-dir target-lite --config 'profile.lite.package.warp.codegen-units=8'
.\target-lite\lite\integration.exe test_lite_multiterminal_stability
.\target-lite\lite\integration.exe test_lite_multiterminal_background_stability
```

每轮创建 4 个标签页、8 个 PowerShell 终端，并发输出 22 秒后各追加 5,000 行，期间切换标签与
调整窗口大小，再关闭 7 个终端并观察资源回落，共重复 3 轮；另等待 65 秒，覆盖默认 60 秒的
撤销关闭保留期。最后验证任务返回非零退出码、
强制终止一个 Shell 后其余 7 个 Shell 继续执行，以及原始终端仍可执行新命令。输出中的
`STABILITY_METRICS` 包含 GUI CPU 时间、帧数、工作集、Windows 私有提交和事件循环采样间隔；
CPU 时间除以实际秒数和逻辑核数，才对应任务管理器中的整机占比。测试设有 2 GiB GUI 内存
保护，不通过耗尽本机内存来验证 OOM。

默认使用 DX12 CPU 软件渲染、最高 15 FPS。设置 `WARP_TEST_STABILITY_AUTOMATIC=1` 可重复
验证自动设备选择与自动帧率；`WARP_TEST_STABILITY_ADAPTIVE=1` 则保持 CPU 渲染，只改用
自适应帧率。设置 `WARP_TEST_STABILITY_BACKGROUND` 为图片的绝对路径，可测试实际
尺寸的背景图。测试会将图片复制到独立数据目录，不修改原文件。短时资源稳定不能证明无限
运行不泄漏；关闭后保留的命令历史、撤销关闭记录及分配器缓存也需与泄漏区分。

Windows 上的标签页、分屏与窗口共享 GUI 进程。强制终止单个 Shell 会使对应分屏只读并保留输出；GUI
主进程崩溃或内存耗尽会影响同一实例的所有会话；ConPTY 关闭后 Shell 也可能延迟退出。
启动阶段的 crash recovery 可以重启界面，但不保留原 Shell 或正在运行的任务，也不等于
运行期间的会话进程隔离。

本机 Windows 11 的 CPU、GPU、背景图和自适应帧率对照结果见
[`stability-windows-2026-11-01.json`](benchmarks/stability-windows-2026-11-01.json)。

## CPU 软件渲染采样验证

CPU 适配器使用显式 texel load 实现与单 mip 线性采样等价的双线性过滤，图片、图标和字形
共用 `software_sampling.wgsl`。硬件适配器继续使用原生采样器。矩形的常量实例数据使用 flat
插值；纯色、无边框、无圆角矩形使用精简管线，并仅合并相邻批次以保留透明叠加顺序。

```powershell
.\target-lite\lite\integration.exe test_lite_software_shader_pixels
$env:WARP_TEST_FORCE_NATIVE_SAMPLING = '1'
.\target-lite\lite\integration.exe test_lite_software_shader_pixels
Remove-Item Env:WARP_TEST_FORCE_NATIVE_SAMPLING
$env:WARP_TEST_STABILITY_AUTOMATIC = '1'
.\target-lite\lite\integration.exe test_lite_software_shader_pixels
Remove-Item Env:WARP_TEST_STABILITY_AUTOMATIC
```

像素测试使用独立的参考窗口，检查渐变、透明叠加顺序、圆角、边框，以及带透明像素图片的
双线性过滤和边缘 clamp；在软件、原生采样对照和硬件三种模式下读取实际渲染结果。
CPU 和内存测量期间应停止编译及其他 GUI 测试，并将 GUI 与 Shell 子进程分开统计。
本机对照数据保存在 [`cpu-software-windows-2026-11-01.json`](benchmarks/cpu-software-windows-2026-11-01.json)。

`WARP_TEST_SKIP_DRAW=rects|glyphs|images`、`WARP_TEST_RECT_STATS` 和
`WARP_TEST_RECT_SHADER` / `WARP_TEST_IMAGE_SHADER` / `WARP_TEST_GLYPH_SHADER` 仅供集成测试
诊断使用；屏蔽绘制的运行不能作为画面正确性验证。替换的 shader 必须包含完整模块声明，
包括 `software_sampling` override。实例面积统计未经裁剪，不能当作实际像素绘制量。

排查参考官方 [#14694](https://github.com/warpdotdev/warp/issues/14694) 的软件渲染高 CPU 报告，
以及 [46c0b51](https://github.com/warpdotdev/warp/commit/46c0b513643a1c9d41b302dbd7601d9f5526502e) 的全进程 CPU 扫描修复。本地也将
PowerShell 历史兼容性检测改为进程名查询，并按应用进程缓存结果，避免每次创建会话都采样
全系统进程的 CPU。

## 资源与提交

保留 `Cargo.lock`、数据库 migrations、ConPTY / OpenConsole、字体、shell bootstrap 与许可证。不要用全局 `*.exe`、`*.dll` 或 `*.txt` 规则排除运行资源。

`target*/`、`dist/`、`.note/` 和排错日志不提交。性能对照的脱敏数值样本保存在
`docs/benchmarks/`；原始日志、会话种子与包副本只留在本机归档。提交前检查
`git diff --check` 和暂存区；不将旧分发包作为当前源码的构建证明。

本仓库默认远端 `origin` 指向 `AnxiangLemon/warp-lite`，不配置自动合并上游。GitHub Actions 的范围见 [.github/workflows/README.md](../.github/workflows/README.md)。
