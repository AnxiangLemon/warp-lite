# Warp Lite

基于 [Warp](https://github.com/warpdotdev/warp) 源码独立维护的本地终端版本，保留真实的终端模型、ConPTY、GPU 渲染、标签分组与分屏，关闭内置 AI、账户登录、计费、云同步和遥测服务。

当前已在 **Windows x64 / MSVC / Rust 1.92.0** 上完成源码构建与真实 GUI 测试。优化构建的 EXE 约 **52 MiB**，具体大小以实际构建为准。macOS、Linux 和 Windows ARM64 尚未完成本轮运行验证。

## 功能

- 本地 PowerShell、CMD、Git Bash、WSL；可用 shell 取决于本机安装情况。
- 标签页、标签分组、固定标签、垂直标签栏、分屏与窗口管理。
- 保存和恢复标签名、目录、分组、分屏及历史输出。
- 本地字体、主题、默认 shell、键盘快捷键和中文设置搜索。
- 可选外置 PNG 背景；未配置时使用主题背景色。

Lite 不配置自动更新、遥测、crash reporting 或静态 MCP 服务，网络 transport 与云凭据运行路径不启用。在终端中自行运行的 CLI 工具仍由该工具决定网络行为。

## 构建与运行

需要 Git、Rust（仓库固定为 `1.92.0`）、MSVC C++ Build Tools、Windows SDK，以及 CMake、Protobuf 和 LLVM / libclang。上游依赖使用 Git LFS 的部分资源，首次 checkout 可执行：

```powershell
git clone https://github.com/AnxiangLemon/warp-lite.git
cd warp-lite
git lfs install
git lfs pull
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

外置背景放在程序旁的 `backgrounds\background.png`，重启后生效；仅支持静态 PNG，文件不超过 8 MiB、边长不超过 4096 像素。解码后纹理会限制到 2048 像素。

本仓库只提交源码和必要资源。构建缓存、`dist/`、日志及本地配置均已忽略。正式分发前还需生成完整第三方依赖许可清单；脚本会报告清单是否齐全。

## 验证与依赖精简

已通过 Lite 编译检查、优化构建、99 项针对性单元测试及 6 项真实 GUI 集成测试，覆盖 PowerShell、中文路径与输出、命令取消、标签分屏、设置搜索、关闭确认和跨进程恢复。完整记录见 [验证说明](docs/validation.md)。

```powershell
# 检查 Lite 编译
cargo check -p warp --bin warp-lite --no-default-features --features warp_lite --locked

# 检查 Windows x64 的 Lite 运行依赖
.\script\verify-lite-dependencies.ps1
```

Lite 已通过 feature 分离移除 AWS SDK、Tantivy、Tink / HPKE、libgit2、QuickJS 和大部分语言解析器等运行依赖；禁用网络、云凭据、MCP transport 与在线遥测路径。仍保留终端、GPU、SQLite、字体与共享数据类型所需组件，部分共享模块会产生 `dead_code` 警告。

更多测试命令与结构说明见 [开发说明](docs/development.md)。反馈和贡献请使用 [本仓库流程](CONTRIBUTING.md)。

## 独立维护与许可

仓库不会自动拉取、合并或发布上游更新；GitHub Actions 仅运行只读的轻量仓库检查。构建依赖仍包含固定版本的上游 Git 库，本项目未做完整 vendoring。

保留原作者与第三方版权声明。`warpui` 与 `warpui_core` 使用 [MIT](LICENSE-MIT)，其余仓库代码使用 [AGPL v3](LICENSE-AGPL)。
