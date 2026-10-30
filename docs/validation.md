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
