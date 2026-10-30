# Warp Lite

基于 [warpdotdev/warp](https://github.com/warpdotdev/warp) 的本地优先终端分支，保留 Warp 的终端模型、GPU 渲染与工作区交互，精简内置 AI、账户、计费与云服务入口。

本仓库提供源码，`dist/` 分发包和 `target*/` 构建缓存不纳入版本控制。当前源码仍在模块拆分阶段，请以本地检查结果为准；已有分发包不能代表当前源码已通过构建。

## 功能范围

- 本地 shell：PowerShell、CMD、Git Bash、WSL，可用项取决于本机安装情况。
- 标签页、标签分组、固定标签、垂直标签栏和分屏。
- 工作区与会话恢复，本地字体、主题、默认 shell 等设置。
- Lite 设置界面的中文标签。
- `WarpLite` 独立应用标识及 `warp-lite.log` 日志名。

Lite 入口不配置 telemetry、crash reporting、自动更新和静态 MCP 服务。Warp 与 Oz 服务配置使用 loopback 端点。不提供内置云 AI 服务；shell 中运行的 CLI 工具仍由相应工具决定网络行为。

## Windows 构建与运行

需要 Git、Rust toolchain（仓库固定为 `1.92.0`）、MSVC C++ Build Tools 与 Windows SDK，以及 CMake、Protobuf、LLVM 等原生构建依赖。首次准备环境可运行上游脚本，它会按需安装开发工具：

```powershell
git clone https://github.com/AnxiangLemon/warp-lite.git
cd warp-lite
.\script\windows\bootstrap.ps1
```

从仓库根目录运行：

```powershell
# 开发运行
.\script\run-lite.ps1

# Release 运行
.\script\run-lite.ps1 -Release

# 仅构建 Lite binary
cargo build -p warp --release --bin warp-lite --no-default-features --features warp_lite --locked

# 查看版本
.\target\release\warp-lite.exe --version
```

`cargo run` 默认入口是上游 `warp-oss`，Lite 必须指定 `--bin warp-lite --no-default-features --features warp_lite`。设置了 `CARGO_TARGET_DIR` 时，产物写入指定目录。

Git Bash 可用 `./script/run-lite` 或 `./script/run-lite --release`。macOS / Linux 依赖安装沿用 `./script/bootstrap`，本分支尚未据此声明跨平台验证通过。

## 开发与文档

- [开发说明](docs/development.md)：源码结构、检查命令、资源与提交规则。
- [贡献指南](CONTRIBUTING.md)：本仓库的反馈与开发流程。
- [工程约定](AGENTS.md)：仓库编码与验证要求。

Lite 通过 `warp_lite` Cargo feature 选择精简模块与兼容接口。源码仍保留上游完整实现，部分共享依赖与数据类型仍参与编译，因此仓库大小和缓存大小不等于最终程序大小。默认 `release` 不等于经过专门体积优化与打包的分发版本。

## 许可与来源

本项目是 Warp 的独立 fork，保留原作者与第三方版权声明。

- `warpui` 与 `warpui_core` 使用 [MIT](LICENSE-MIT)。
- 其余仓库代码使用 [AGPL v3](LICENSE-AGPL)。

分发时同时保留对应许可证、第三方许可与所需运行资源。

本次源码检查结果见 [验证记录](docs/validation.md)。
