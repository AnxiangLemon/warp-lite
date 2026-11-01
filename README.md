# Warp Lite

基于 [Warp](https://github.com/warpdotdev/warp) 源码独立维护的本地终端版本，保留真实的终端模型、ConPTY、GPU 渲染、标签分组与分屏，关闭内置 AI、账户登录、计费、云同步和遥测服务。

当前已在 **Windows x64 / MSVC / Rust 1.92.0** 上完成源码构建与真实 GUI 测试。`0.1.0` 的优化构建 EXE 约 **50.87 MiB**，Windows x64 下载包约 **21.46 MiB**。macOS、Linux 和 Windows ARM64 尚未完成本轮运行验证。

## 下载与使用

普通用户直接到 [GitHub Releases](https://github.com/AnxiangLemon/warp-lite/releases/latest) 下载，无需安装 Rust 或自行编译。

1. 打开 [0.1.0 发布页](https://github.com/AnxiangLemon/warp-lite/releases/tag/0.1.0)，下载 **`warp-lite-0.1.0-windows-x64.zip`**。GitHub 自动提供的 `Source code` 是源码包。
2. 将 ZIP **完整解压**到本地目录，打开其中的 `Warp Lite` 文件夹。
3. 双击 `warp-lite.exe` 启动；需要使用 Git Bash、WSL 或 PowerShell 7 时，先在本机安装对应 shell。

本次预编译包面向 **Windows x64**，已在 Windows 11 上验证。不提供 macOS、Linux 或 ARM64 预编译包。
包内只保留程序、必要运行库、shell 集成脚本和许可证；不附带背景图、独立图标、开发工具、构建缓存或调试符号。请保持解压后的目录结构，不能只取出 EXE 运行。

更新时关闭 Warp Lite，再从 Releases 下载新版本并完整解压。Lite 不自动更新，背景图等自行添加的文件请自行保留。配置和会话数据由应用写入用户数据目录，不代表所有数据都保存在 EXE 旁。

## 功能

- 本地 PowerShell、CMD、Git Bash、WSL；可用 shell 取决于本机安装情况。
- 标签页、标签分组、固定标签、垂直标签栏、分屏与窗口管理。
- 保存和恢复标签名、目录、分组、分屏及历史输出。
- 本地字体、主题、默认 shell、键盘快捷键和中文设置搜索。
- 独立 Lite 图标与信息页，字体列表和额外图形后端按需加载。
- 可选择 GPU 优先或 CPU 软件渲染，支持图形后端选择和帧率上限。
- 可选外置 PNG / JPEG 背景，在后台加载；未配置时使用主题背景色。

Lite 不配置自动更新、遥测、crash reporting 或静态 MCP 服务，网络 transport 与云凭据运行路径不启用。在终端中自行运行的 CLI 工具仍由该工具决定网络行为。

## 自定义背景与图形设置

背景图是可选配置，发布包不附带图片。未配置时使用主题背景色。

1. 在 `warp-lite.exe` 所在目录新建 `backgrounds` 文件夹。
2. 放入自己的静态 PNG / JPEG，命名为 `background.png`、`background.jpg` 或 `background.jpeg`。
3. 重启 Warp Lite。

例如：

```text
Warp Lite/
├── warp-lite.exe
├── backgrounds/
│   └── background.jpg
└── …其他随包运行文件
```

按上述文件名顺序使用第一张有效图片，文件格式按内容识别。单张文件不超过 **8 MiB**、边长不超过 **4096 像素**，解码后的纹理限制到 **2048 像素**。图片在后台加载，窗口先显示主题底色，再显示背景；替换图片后需要重启。

在“设置 → 外观 → 图形渲染”中可选择渲染设备（自动 / CPU 软件渲染）、图形后端（默认 / DirectX 12 / Vulkan / OpenGL）和帧率上限。设备、后端修改后新建窗口或重启生效，帧率设置立即生效。默认优先使用 GPU；没有可用 GPU 时回退到 CPU，实际可用后端取决于驱动。使用 CPU 软件渲染时可按需选择 15 / 30 FPS。

## 从源码构建与运行

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

`0.1.0` 发布包使用 Windows 系统 shader compiler，不额外附带 DXC DLL。完整第三方依赖许可清单位于包内 `resources/THIRD_PARTY_LICENSES.txt`，单项许可证位于 `licenses/`，项目许可证位于包根目录。

本仓库只提交源码和必要资源。构建缓存、`dist/`、日志及本地配置均已忽略。正式发布的二进制 ZIP 通过 GitHub Releases 分发；自行打包时应确认脚本报告 `ThirdPartyLicensesIncluded = True`。

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
