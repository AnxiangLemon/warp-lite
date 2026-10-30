# Warp Lite 验证记录（2026-10-30）

本轮已修复此前开发快照的编译错误，恢复完整的共享模块与 feature 边界，移除不完整的替代模块，并在当前仓库目录重新构建和运行验证。

## 构建环境与产物

- 平台：Windows x64 / MSVC，Rust `1.92.0`。
- 构建：`cargo build -p warp --profile lite --bin warp-lite --no-default-features --features warp_lite --locked`，成功。
- 优化 EXE：约 52 MiB，源码目录为当前 checkout；不是沿用旧 `dist/` 程序。
- EXE SHA256：`D3F715C47B2A9413C6912A622BEB1FE03AC03AB106B274050E6EEFB26F2069B3`。
- 从其他工作目录执行 `script/run-lite.ps1 -Release -BuildOnly`，退出码 `0`。
- `script/package-lite.ps1 -SkipBuild -SystemShaderCompiler` 本地打包成功，新 EXE `--version` 输出 `Warp Lite 0.1.0`，退出码 `0`。

本地验证包包含运行 DLL、shell 脚本和单项许可证，但尚未生成完整的 Cargo 依赖第三方许可清单，因此不作为正式二进制发行包发布。本轮向仓库提交源码，`dist/` 不提交。

## 静态检查

| 检查 | 结果 |
| --- | --- |
| `./script/format` / `--check` | 通过 |
| `script/check_no_inline_test_modules` | 通过 |
| workspace Clippy，排除 `warp_completer`，all targets / tests，`-D warnings` | 通过 |
| `cargo clippy -p warp --all-targets --tests -- -D warnings` | 通过 |
| `cargo clippy -p warp_completer --all-targets --tests -- -D warnings` | 通过 |
| PowerShell parser、Bash syntax、文档相对链接 | 通过 |
| `git diff --check` 与构建产物排除检查 | 通过 |

Lite 优化构建仍有共享模块的 `dead_code` 警告；未通过全局关闭 lint 隐藏这些警告。上表三组严格 Clippy 使用 `script/presubmit` 指定的 feature 配置。

## 针对性单元测试：99 项通过

- `http_client`、`warp_server_client`、`warp_managed_secrets`、`mcp` 的离线配置：45 项。
- Lite CLI、中文文案和相关 Lite 测试：40 项。
- 终端持久化类型兼容：4 项。
- 外置 PNG 的格式、尺寸、文件替换与错误处理：5 项。
- 禁用遥测时不构造、不发送、不改写事件文件：3 项。
- SQLite 侧栏状态 round trip：1 项。
- MCP capabilities 的独立查询回归：1 项。

本机没有 `cargo-nextest`，本轮使用 `cargo test` 和其生成的测试程序执行以上测试，未宣称运行完整 workspace test suite。

## 实际 GUI 集成测试：6 项通过

测试使用当前源码构建的 `integration` binary、`warp_lite` feature 和真实显示模式，并使用独立的 `WarpIntegrationLiteSmoke-*` 应用标识与数据目录。

| 测试 | 验证内容 |
| --- | --- |
| `test_lite_terminal_smoke` | PowerShell、Git、中文与空格路径、中文输出、取消命令、SQLite 快照及真实渲染截图 |
| `test_lite_terminal_management` | 标签、分组、分屏和本地终端管理 |
| `test_lite_chinese_settings` | 中文设置文案及搜索 |
| `test_lite_restore_named_tabs` | 命名标签恢复 |
| `test_lite_restore_saved_tabs` | 新进程恢复已保存的目录、分组、侧栏与历史输出 |
| `test_lite_close_confirmation` | 运行中命令的关闭确认 |

跨进程恢复使用 SQLite backup API 生成一致性备份后验证。直接复制 WAL 模式主文件曾导致测试输入缺少最新 schema；使用一致性备份后通过。

## 依赖检查与范围

`script/verify-lite-dependencies.ps1` 通过：Windows x64 Lite 运行依赖共 652 个 package / version 组合，没有发现检查规则禁止的服务依赖。AWS SDK、Tantivy、Tink / HPKE、libgit2、QuickJS 与完整语言解析器等不在 Lite 运行图中。仍有 HTTP / TLS 的共享类型与库依赖，不能据此宣称所有网络相关代码都已删除。

macOS、Linux、Windows ARM64 未进行本轮构建或运行验证。本机没有 `oz-dev`，无法发现并使用云端 runner；本轮结论限定在本地 Windows x64。

验证日志、截图和依赖审计 JSON 留在被忽略的 `.note/publish/` 中；可复现命令见 [开发说明](development.md)。

## 2026-10-31：Windows 内存与程序标识

- 同机、相同 Debug 构建类型、1280 × 800 窗口、独立数据配置、单终端的对照：
  空终端私有工作集约 342 → 163 MiB；打开设置后 344.35 → 188.54 MiB（下降 45.2%）。
  字体枚举调整本身仅节省约 1 MiB，主要下降来自图形内存池改用按需增长策略。
- 数值来自 Windows `WorkingSetPrivate`，不等于总工作集或私有提交量，不包含 shell 和
  崩溃恢复辅助进程；窗口数、输出历史、字体和显卡驱动都会影响实际占用。
- 优化 EXE 为 54,365,696 字节（51.85 MiB），本地包约 53.86 MiB。优化版恢复当前
  4 个终端及设置页后，主进程私有工作集采样为 208.33 MiB；这不是单终端对照的同一场景。
- 优化版 SHA256：`28BC90F12F3FAD167CCF10CC1E94C8ECD90EED5591F1084065886F2503DC5FBA`。
  已更新 `dist/Warp Lite Verified`；EXE 的产品名、描述、版本与运行窗口图标均核对正确。
- 字体排版单元测试 10 项通过；真实显示的终端管理回归 20 步、中文设置回归 21 步通过。
  字体按需加载后手动切换 Consolas 正常。依赖审计仍为 652 个 package / version，无禁止项。
- 修改文件的 Rust 格式、PowerShell 语法及 `git diff --check` 通过；没有重跑完整 workspace
  Clippy。构建仍有既有 Lite 共享模块的 unused / dead-code 警告。
- 测量、构建日志和截图保存在 `target/lite-memory-*`，汇总为
  `target/lite-memory-verification.json`。本轮运行验证仅覆盖 Windows x64。

## 2026-10-31：继续缩减 DirectX 预留内存

- Windows `VirtualQueryEx` / `QueryWorkingSetEx` 显示，上一版仍有一块 64,004,096 字节的
  常驻私有分配。对照当前 wgpu 30 的 D3D12 实现，`max_non_sampler_bindings` 默认值为
  1,000,000，会在创建设备时分配描述符堆；`MemoryHints::MemoryUsage` 不控制这项预留。
- Windows Lite 将该值设为 65,536；保留原有图形后端选择、回退和图形内存池策略。
  这项设置不改变终端历史、字体质量或会话数量。
- 同机、相同 Debug 构建类型、1280 × 800 窗口、独立配置的单终端对照：
  空终端私有工作集 163.15 → 105.85 MiB，设置页 188.54 → 约 131.08 MiB；
  新进程内存映射中已无原来的 61 MiB 常驻私有分配。
- 独立 `dhat_heap_profiling` 构建的单终端诊断快照：存活 Rust 堆为 20,831,583 字节，
  峰值为 103,710,725 字节；这些数值不包含原生图形驱动分配，也不等于进程工作集。
  发布构建未启用该分析 feature。快照为 `target/dhat-heap.json`。
- 当前源码的真实显示回归通过：`test_lite_terminal_smoke` 21 步、
  `test_lite_terminal_management` 20 步、`test_lite_chinese_settings` 21 步。
  已检查终端截图中的中文输出、命令结果及光标。格式检查通过；未运行完整 workspace Clippy。
- 采样为 `target/lite-memory-descriptors-{empty,settings}.json`，构建、测试日志与截图在
  `target/lite-memory-descriptors-*`。验证范围为 Windows x64。
- 已更新并启动 `dist/Warp Lite Verified/warp-lite.exe`。恢复现有 4 个终端和设置页后，
  连续 5 次采样私有工作集均为 158.44 MiB；本轮更新前同一会话观测值为 214.45 MiB。
  新版总工作集约 270.22 MiB，私有提交量 213.77 MiB；不能混用这三个计数器。
  原始采样为 `target/lite-memory-descriptors-package-runtime.json`。
  EXE 仍为 54,365,696 字节，SHA256 为
  `A391AEA1C48B6847C9D635B7626C472E63E121E41ECE3991BA396DFD3A8DB6DA`。

## 2026-10-31：字体、图形后端按需加载与启动优化

- Windows Lite 默认仅创建 DirectX 实例。默认终端、外观和终端设置页均实测未加载
  `vulkan-1.dll`；显式 Vulkan、OpenGL 及现有硬件兼容回退路径仍保留。
- SVG 纯路径图标不再触发系统字体数据库初始化，文本 SVG 使用原有系统字体和内置回退。
  设置页只在首次展开字体选择器时异步枚举字体；首次展开后搜索、切换 Consolas 已手动验证。
- 相同 Debug 类型、1280 × 800 窗口、单终端的阶段采样：空闲私有工作集约 79.29 MiB，
  打开外观设置约 103.82 MiB，展开系统字体列表后约 106.50 MiB。对应私有提交量分别约
  127.86、154.37、156.36 MiB。未调用工作集清理或内存压缩 API。
- `image_cache::tests` 19 项通过，包括纯路径 SVG 不加载字体、文本、系统字体及内置字体
  回退。`warpui` / `warpui_core` 的 library Clippy（`-D warnings`）通过；修改文件的
  Rust 格式检查通过。未重跑完整 workspace Clippy。
- 默认真实显示回归：终端 smoke、终端管理、中文设置通过；强制 Vulkan smoke 通过，
  并检查了中文输出截图。测试辅助代码改用 `$PSVersionTable.PSVersion.ToString()` 获取
  版本，兼容 Windows PowerShell 5.1 和 PowerShell 7。
- 强制 OpenGL 完成终端功能及持久化断言，但最后的内置截图步骤因 surface 不支持
  `COPY_SRC` 失败，因此不计为整项测试通过。另起隔离的 Debug 进程，用桌面截图核对了
  OpenGL 终端及中文设置的实际渲染。
- 本轮运行验证限定 Windows x64；本机无可用 `oz-dev` 云 runner，未验证其他平台。
  采样、构建和测试日志保存在 `target/lite-deep-*`。
- 最终同条件对照采用两份 Debug 包、相同 runtime DLL、同一隔离配置及 1280 × 800
  窗口，在编译结束后分别预热一次、测量四次。启动后 5 秒的主进程私有工作集中位数为
  **106.14 → 77.51 MiB（下降约 27.0%）**；私有提交量为 **158.91 → 125.89 MiB**。
  两者都下降，不能解释成仅把驻留页面移出工作集。
- 从启动进程到窗口可见且解除 DWM cloak 的热启动中位数为 **133.8 → 133.6 ms**；
  差异处于波动范围，不宣称首帧显著加速。这不是冷启动或 shell 完成启动的时间。
  原始数据为 `target/lite-deep-startup-packaged-{before,after}.json`，汇总为
  `target/lite-deep-startup-summary.json`，测量脚本为
  `target/benchmark-lite-debug-startup.ps1`（仅用于这两份隔离的 Debug 构建）。
- 最新 `lite` 优化构建成功，EXE **53,219,328 字节（50.75 MiB）**，相较上一版减少
  1,146,368 字节；本地包 **55,328,694 字节（52.77 MiB）**。仅调整主应用 codegen units，
  不改变终端核心的优化级别。SHA256：
  `E4866743120E9390CCF2D63F2CB307B6D998677A5C9AB0D016EAF323638A2160`。
- 已更新并启动 `dist/Warp Lite Verified/warp-lite.exe`，核对产品名和描述为 Warp Lite。
  实际恢复 4 个终端标签页（包含分屏）及设置页后，5 次私有工作集采样为
  **121.69–121.75 MiB**，总工作集为 **193.28–193.34 MiB**，私有提交量为
  **172.01–172.07 MiB**；进程未加载 `vulkan-1.dll`。这些是实际会话的独立采样，
  不与前述单终端 Debug 对照混用。记录为 `target/lite-deep-package-settings.json`。
- 最终 `git diff --check` 通过，未提交或推送源码。

## 2026-10-31：退出时查看运行进程的焦点修复

- 复现：在终端中运行命令，点击退出确认的“查看进程”，列表立即消失；设置页能够显示。
  修复前真实显示回归明确失败于 `View running processes from a terminal tab`，
  `focused=false, bounds=None`，日志为 `target/lite-process-list-before.log`。
- 原因：进程列表打开后，`NativeModalEvent::Close` 无条件把焦点交还 Workspace，
  随后终端重新获得焦点并关闭列表。现在按当前可见界面恢复焦点：进程列表或切换列表已打开
  时聚焦该列表，否则恢复当前标签页。
- 扩充 `test_lite_close_confirmation`，使用真实点击“查看进程”按钮验证终端页和设置页，
  同时断言列表有可见边界、结果及键盘焦点；覆盖 Escape、Enter 跳转到运行终端、
  列表上再次退出并取消、继续取消退出及关闭普通标签页。检查过程中运行命令不被终止。
- 修复后真实显示回归 30 步通过，检查了两种页面的实际截图；修改文件的 Rust 格式检查及
  `git diff --check` 通过。验证范围为本地 Windows x64，未重跑完整 workspace Clippy。
  日志为 `target/lite-process-list-fixed.log`，截图在
  `target/lite-process-list-fixed-artifacts/test_lite_close_confirmation/`。
- 最新 `lite` 优化构建及打包成功，修复包位于 `dist/Warp Lite Process Fix/warp-lite.exe`，
  EXE 为 53,219,328 字节（50.75 MiB），`--version` 输出 `Warp Lite 0.1.0`，SHA256 为
  `82B00F2CB076E3D9DFB9E14CE7D56677A89454CF387133D6D9528937D12CB735`。
  用户的 `dist/Warp Lite Verified` 旧版仍在运行，因此未覆盖或终止该进程；修复包使用相同
  应用标识，退出旧版后可继续读取原来的配置与会话记录。

## 2026-10-31：JPEG 背景与 WSL 启动阻塞

- 背景加载原本只查找 `background.png` 并调用 PNG decoder。现在依次使用第一张有效的
  `background.png` / `background.jpg` / `background.jpeg`，按内容识别格式，JPEG 应用
  EXIF 方向。保留 8 MiB 文件、4096 像素输入和 2048 像素纹理限制。
- 背景读取、解码和缩放移到 AppearanceManager 的后台任务。首屏使用主题底色，加载完成后
  刷新当前主题；切换主题、替换或删除源文件不会改变本次运行的背景快照。
- 用户会话恢复日志在第一、第二个终端之间有约 3 秒空档。读取隔离快照中的目录并逐一检查，
  首个 WSL 目录的 `is_dir` 实测 2425.7 ms；终端模型和视图自身的诊断仅为数毫秒。
  恢复目录的检查现移到 shell 后台任务，WSL HOME 查询改为异步，Windows 启动器不再把
  WSL UNC 路径作为自身的工作目录，Linux cwd 继续通过 shell bootstrap 恢复。
- 图片单元测试 10 项通过：PNG/JPEG、内容探测、EXIF 旋转、尺寸限制、损坏 PNG 回退 JPEG、
  PNG 优先级、删除或替换文件后保留快照。真实显示测试通过 JPEG、PNG、缺失、损坏、
  超大背景、三个命名标签页目录恢复、WSL 中文目录搜索与跳转，已检查 JPEG、PNG 切换主题及
  WSL 截图。
- 原有 WSL 测试未隔离 Linux HOME，读取真实用户 rcfiles 时首条命令混入控制字符，两次
  诊断失败均保留。测试现在通过 `WSLENV=HOME/p` 使用生成的测试 rcfiles，并成功通过；
  此结果不等同于验证所有用户自定义 shell 初始化脚本。
- 热启动对照使用相同 Debug 构建类型、runtime DLL、5 个终端的会话快照、设置和同一张
  1920×1080 PNG，先预热一对、再交替测量三对，测量时无编译进程。实际首帧回调的中位数为
  **1077 → 1106 ms**，五个 shell 就绪（100 ms 日志轮询观测）为 **2614.9 → 2613.4 ms**。
  没有显著热启动改善，不宣称整体启动加速；本轮移除了背景处理和 WSL 冷访问对 UI 的同步
  阻塞。未关闭 WSL、清缓存或绕过用户启动脚本来制造更好的数据。
- 原始数据、脚本及汇总在 `target/lite-jpeg-startup-benchmark.json`、
  `target/benchmark-lite-restored-startup.ps1`、`target/lite-jpeg-startup-summary.json`。
  性能探针先停止自身的崩溃恢复 helper，再停止主进程，避免自动重启污染后续测量。
  测量用 Debug 二进制与最终源码仅存在计时类型名称差异；Windows 上 `instant::Instant`
  直接重导出 `std::time::Instant`，运行逻辑相同。
- 验证范围为 Windows x64；未安装 `oz-dev` 和 `cargo-nextest`，使用本地 `cargo test`
  与真实显示 integration runner。运行依赖检查仍为 652 个 package/version，无新增禁止依赖。
- 修改文件的 Rust 格式检查与 `git diff --check` 通过；Lite binary Clippy 使用
  `-D clippy::correctness -D clippy::disallowed_types` 通过，仍有 2099 条既有 Lite 警告。
  未把该结果计为完整 workspace 的 `-D warnings` 通过。
- 最终 `lite` 优化构建成功，新包位于 `dist/Warp Lite Startup Fix/warp-lite.exe`。
  EXE 为 53,262,848 字节（50.80 MiB），基础包约 52.81 MiB；复制用户已有 PNG/JPG 后
  约 56.37 MiB。两张图片均校验哈希一致，原目录未改动，仍按 PNG 优先的规则加载。
  通过等待子进程并捕获输出核对 `--version` 为 `Warp Lite 0.1.0`，exit code 0；直接在
  PowerShell 中捕获 Windows GUI EXE 输出会提前关闭管道，第一次核验未计为成功。
  构建与打包的 EXE SHA256 一致：
  `A97CCA7442213D45D7C33C325DD5E971399BC3658112CE7070D34AE15AEC69EC`。
  包信息保存在 `target/lite-jpeg-startup-package.json`，未提交或推送源码。

## 2026-10-31：合并现有修复与依赖本地化

本轮纳入 02:57 固定的 56 文件修复快照，并完成必要 Git 依赖本地化、无用前端 / 模型 /
PDB 与 LFS 占位测试资源清理、transport 分离及开发脚本独立化。重新验证的结果、产物
SHA256、142 项单元测试、12 个 GUI 用例与 Vulkan smoke test，见
[独立维护说明](independence.md#本轮验证记录2026-10-31)。此前单项实验的旧 SHA256 和
内存采样保留为历史记录，不代表本次构建的产物。当前验证包为
`dist/Lite Independence Snapshot/`，EXE 50.79 MiB，源码和正式二进制发布范围见该记录。
