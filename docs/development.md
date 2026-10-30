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
```

测试使用独立的 `WarpIntegrationLiteSmoke-*` 应用标识与数据目录，不使用日常 Warp Lite 数据库。实际显示测试逐个运行，避免同时争用窗口和焦点。

跨进程恢复使用 `test_lite_restore_saved_tabs`。先从 smoke test 输出的 evidence JSON 获取数据库与项目目录，将 SQLite 一致性备份路径设置为 `WARP_LITE_RESTORE_DATABASE`、项目目录设置为 `WARP_LITE_RESTORE_DIRECTORY`，再执行该测试；设置 `WARP_LITE_EXPECT_RESTORED_GROUPS=1` 还会检查分组和侧栏恢复。SQLite 使用 WAL，不能仅复制仍有 WAL 数据的主数据库文件；应使用 SQLite backup API，或在确认进程退出并 checkpoint 后复制。

## 资源与提交

保留 `Cargo.lock`、数据库 migrations、ConPTY / OpenConsole、字体、shell bootstrap 与许可证。不要用全局 `*.exe`、`*.dll` 或 `*.txt` 规则排除运行资源。

`target*/`、`dist/`、`.note/`、排错日志和一次性 import 改写脚本不提交。提交前检查 `git diff --check` 和暂存区；不将旧分发包作为当前源码的构建证明。

本仓库默认远端 `origin` 指向 `AnxiangLemon/warp-lite`，不配置自动合并上游。GitHub Actions 的范围见 [.github/workflows/README.md](../.github/workflows/README.md)。
