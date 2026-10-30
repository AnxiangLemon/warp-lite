# Warp Lite

基于 [Warp](https://github.com/warpdotdev/warp) 源码独立维护的本地终端版本，保留真实的终端模型、ConPTY、GPU 渲染、标签分组与分屏，关闭内置 AI、账户登录、计费、云同步和遥测服务。

当前已在 **Windows x64 / MSVC / Rust 1.92.0** 上完成源码构建与真实 GUI 测试。优化构建的 EXE 约 **50.79 MiB**，具体大小以实际构建为准。macOS、Linux 和 Windows ARM64 尚未完成本轮运行验证。

## 功能

- 本地 PowerShell、CMD、Git Bash、WSL；可用 shell 取决于本机安装情况。
- 标签页、标签分组、固定标签、垂直标签栏、分屏与窗口管理。
- 保存和恢复标签名、目录、分组、分屏及历史输出。
- 本地字体、主题、默认 shell、键盘快捷键和中文设置搜索。
- 独立 Lite 图标与信息页，字体列表和额外图形后端按需加载。
- 可选外置 PNG / JPEG 背景，在后台加载；未配置时使用主题背景色。

Lite 不配置自动更新、遥测、crash reporting 或静态 MCP 服务，网络 transport 与云凭据运行路径不启用。在终端中自行运行的 CLI 工具仍由该工具决定网络行为。

## 构建与运行

需要 Git、Rust（仓库固定为 `1.92.0`）、MSVC C++ Build Tools、Windows SDK，以及 CMake、Protobuf 和 LLVM / libclang。当前源码和运行资源均随仓库保存，无需 Git LFS；只需当前版本时可浅克隆：

```powershell
git clone --depth 1 https://github.com/AnxiangLemon/warp-lite.git
cd warp-lite
```

缺少原生开发工具时，可用仓库内的 Windows 初始化脚本按需安装；它会安装一套较完整的开发环境：

```powershell
.\script\windows\bootstrap.ps1
```

从仓库根目录运行 Lite：

```powershell
# 开发运行
.\script\run-lite.ps1

# 优化构建并运行
.\script\run-lite.ps1 -Release

# 仅构建优化版
.\script\run-lite.ps1 -Release -BuildOnly
```

直接使用 Cargo 的等价命令：

```powershell
cargo build -p warp --bin warp-lite --profile lite --no-default-features --features warp_lite --locked
.\target\lite\warp-lite.exe
```

`lite` profile 使用体积优化、ThinLTO 和符号剥离。设置 `CARGO_TARGET_DIR` 时，输出目录相应改变。首次构建需要编译较多共享 UI 组件，耗时明显长于增量构建。

**不要直接用不带参数的 `cargo run` 构建 Lite**：仓库仍保留完整版本的默认 feature 配置。Lite 必须使用上述脚本，或显式指定 `--bin warp-lite --no-default-features --features warp_lite`。

Git Bash 对应命令为 `./script/run-lite --release`，仅构建时增加 `--build-only`。

## Windows 打包

```powershell
# 使用系统 shader compiler，生成 dist\Warp Lite
.\script\package-lite.ps1 -SystemShaderCompiler
```

打包脚本收集 EXE、ConPTY / OpenConsole、VC runtime、shell 集成脚本和许可证。没有指定 `-SystemShaderCompiler` 时，还会收集 DXC runtime。不要只复制 EXE 作为分发包。

外置背景放在程序旁的 `backgrounds` 目录，依次查找 `background.png`、`background.jpg`、`background.jpeg`，使用第一张有效图片，重启后生效。支持静态 PNG / JPEG，按文件内容识别格式；文件不超过 8 MiB、边长不超过 4096 像素，解码后纹理限制到 2048 像素。窗口先显示主题底色，后台完成图片加载后自动显示背景，不阻塞首屏。

本仓库只提交源码和必要资源。构建缓存、`dist/`、日志及本地配置均已忽略。正式分发前还需生成完整第三方依赖许可清单；脚本会报告清单是否齐全。

## 验证与依赖精简

本轮合入 02:57 的现有修复快照后，通过优化构建、142 项针对性单元测试和 12 个真实 GUI 用例，并额外通过显式 Vulkan 的终端 smoke test。覆盖 PowerShell、WSL 目录搜索、中文设置、会话恢复、关闭确认及 PNG/JPEG 异步背景。结果及验证范围见 [本轮验证记录](docs/independence.md#本轮验证记录2026-10-31)，此前单项记录见 [验证说明](docs/validation.md)。

```powershell
# 检查 Lite 编译
cargo check -p warp --bin warp-lite --no-default-features --features warp_lite --locked

# 检查本地源码来源与完整性（需要 Python 3.9+ 及已缓存的 Cargo registry）
python .\script\verify-local-sources.py

# 检查 Windows x64 的 Lite 运行依赖
.\script\verify-lite-dependencies.ps1
```

Lite 已通过 feature 分离移除 AWS SDK、TLS / AWS-LC、ONNX / Candle、Tantivy、Tink / HPKE、libgit2、QuickJS 和大部分语言解析器等运行依赖；禁用网络、云凭据、MCP transport 与在线遥测路径。仍保留终端、GPU、SQLite、字体与共享数据类型所需组件，部分共享模块会产生 `dead_code` 警告。

更多测试命令与结构说明见 [开发说明](docs/development.md)。反馈和贡献请使用 [本仓库流程](CONTRIBUTING.md)。

## 独立维护与许可

仓库不会自动拉取、合并或发布上游更新；GitHub Actions 仅运行只读的轻量仓库检查。必要的 Git 源码依赖已保存到 `vendor/upstream/`，Cargo 不再从上游 Git 仓库取源码。普通 crates.io 依赖仍通过 `Cargo.lock` 固定，首次构建需要下载或已有本地缓存。来源、固定 commit、许可证和逐文件 SHA256 见 [`vendor/sources.json`](vendor/sources.json)，清理范围和复现方法见 [独立维护说明](docs/independence.md)。

保留 Git 历史用于追溯来源；历史贡献者不表示当前维护权限，也不表示会接收上游更新。只需要 Lite 当前代码时，浅克隆即可减少历史下载。

保留原作者与第三方版权声明。延续原仓库的许可划分：`warpui` 与 `warpui_core` 使用
[MIT](LICENSE-MIT)，其余原仓库代码使用 [AGPL v3](LICENSE-AGPL)。`vendor/upstream/`
中的第三方源码遵循各包自带许可证，声明与来源见 [`vendor/sources.json`](vendor/sources.json)。
