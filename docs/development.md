# Warp Lite 开发说明

## 源码结构

| 路径 | 用途 |
| --- | --- |
| `app/src/bin/lite.rs` | Lite 入口，设置独立应用标识、本地服务配置与日志名 |
| `app/Cargo.toml` | `warp-lite` binary 与 `warp_lite` feature 聚合 |
| `app/src/lib.rs` | 共享应用入口，按 feature 选择完整或 Lite 模块 |
| `app/src/ai_lite/`、`app/src/ai_assistant_lite/`、`*_lite.rs` | Lite 接口与精简实现，仍为共享代码保留部分数据类型 |
| `app/src/terminal/` | 终端模型、输入、渲染与 shell 集成 |
| `app/src/workspace/`、`app/src/pane_group/` | 工作区、标签与分屏 |
| `app/src/settings_view/lite_i18n.rs` | Lite 设置中文文案映射 |
| `crates/warp_core/src/channel/config.rs` | `local_only()` 服务配置 |
| `crates/warpui/`、`crates/warpui_core/` | 共享 UI 基础设施 |
| `script/run-lite`、`script/run-lite.ps1` | 本地运行脚本 |

修改模块门控时同时检查默认上游配置和 Lite 配置，避免一个配置能编译、另一个引用缺失模块。

## 本地验证

先执行最小构建检查：

```powershell
cargo check -p warp --bin warp-lite --no-default-features --features warp_lite --locked
```

格式化与上游 Clippy 门槛以 `script/presubmit` 为准，在 Git Bash 中运行：

```bash
./script/format
./script/format --check
cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings
cargo clippy -p warp --all-targets --tests -- -D warnings
cargo clippy -p warp_completer --all-targets --tests -- -D warnings
```

Lite 自身还应检查其 feature 组合：

```powershell
cargo clippy -p warp --bin warp-lite --no-default-features --features warp_lite -- -D warnings
```

完整检查使用 `./script/presubmit`，包含格式、Clippy、原生代码格式和测试。按改动范围运行对应测试；GUI 行为需要实际启动观察，不能用 `cargo check` 替代。明确记录尚未运行或失败的检查，不把旧构建日志当作当前源码验证结果。

## 资源与构建产物

以下内容留在本机，由 `.gitignore` 过滤：

- 根目录 `target/`、`target-*/`、`dist/`：编译缓存和分发产物。
- 根目录 `lite_check*.txt`：历史排错日志。
- `script/fix_lite_document_imports.py` 与 `script/fix_lite_drive_imports.py`：本次拆分使用的一次性源码改写脚本，不属于运行或构建依赖。
- `.note/`、Python cache、个人环境变量文件与编辑器临时文件。

过滤不等于删除本机文件。不要用全局 `*.exe`、`*.dll`、`*.txt` 或图片规则批量排除：上游受控资源包含 Windows ConPTY / OpenConsole 组件、字体许可证、shell bootstrap 文本和测试数据，它们可能是运行或编译依赖。

保留 `Cargo.lock` 以复现依赖版本。仓库使用 Git LFS 管理部分 PDB 与模型资源，新 checkout 应安装 Git LFS 并执行 `git lfs pull`。不要将 LFS pointer 当作实际资源打包。

## 提交与同步

提交前检查 `git status --short`、`git diff --check` 与暂存区内容，确认没有缓存、分发目录或本地配置。源码拆分所需的 `*_lite.rs` 和 `*_full.rs` 需要一并提交，不能仅提交引用它们的模块声明。

目标仓库为 `AnxiangLemon/warp-lite`，默认分支为 `main`。本地默认远端 `origin` 指向本仓库；
上游历史仅用于追踪来源，仓库不配置自动拉取或合并上游更新。推送使用正常的 fast-forward
或 merge 流程，保留远端已有提交。

## 自动化范围

仅保留轻量的 `Repository checks`，检查变更行的 whitespace 与误提交的构建产物。
移除上游私有 runner、云端发布、同步、Oz 机器人和通知等自动化依赖。
具体触发条件与权限见 [.github/workflows/README.md](../.github/workflows/README.md)。

这项清理不改变 `Cargo.toml` / `Cargo.lock` 中已有的编译依赖；其中仍包含由上游维护的 Git
库。完全独立的依赖镜像或 vendoring 需要另行验证，不能通过删除工作流实现。
