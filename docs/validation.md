# 源码整理验证记录（2026-10-30）

本次整理补充 README 与开发说明，过滤本地构建产物、排错日志及一次性迁移脚本，并收集此前未提交的 Lite 模块拆分代码。

## 已完成

- `.gitignore` 验证：`target-lite/`、`dist/`、`lite_check*.txt` 和两个一次性 import 改写脚本均被排除；`.env.example` 可继续跟踪。
- 暂存区 `git diff --cached --check` 通过；修正六个新文件的行尾或末尾空行。
- 暂存区未包含上述本地产物，保留编译资源、许可证与 `Cargo.lock`。

## 发布阻塞

- `./script/format` 失败：`app/src/ai_lite/facts/manager.rs` 不存在，无法解析 `manager` module。
- `cargo check -p warp --bin warp-lite --no-default-features --features warp_lite --locked` 报错：`facts` 缺失 `manager` 与 `view` module，同时存在 `crate::ai::agent::Shared` 未解析及 Lite 兼容接口缺失等错误。
- `cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings` 失败：`rquickjs-sys v0.3.1` 的 build script 找不到 `patch` 工具。其余 Clippy 组合与完整 presubmit 未运行。
- Lite 编译已报告 3220 个错误与 245 个警告，确认失败后停止编译器；该数量是本次输出记录，不代表完整问题清单。`cargo check` 返回 101。
- 未验证 GUI 行为或 macOS / Linux 构建。旧 `dist/` 分发包与历史成功日志不作为当前源码可构建的证据。

应先完成 Lite 接口对齐和缺失模块修复，再重新执行格式化、Clippy 与相关测试。当前整理结果属于开发中的源码快照，不是已验证的发布版本。

## 独立仓库自动化清理

本次仅修改 `.github/` 配置和文档，未修改 Rust 源码、依赖或运行资源。
删除上游专用 workflows、actions、维护脚本、发布配置及私有 Dependabot 配置，
仅保留本仓库的只读 `Repository checks`，Issue / PR 模板与手动 triage 提示。

已完成验证：

- 所有保留 YAML 与 JSON 可解析，文档相对链接有效。
- 自动化仅有 push / PR / 手动触发，权限为 `contents: read`；不包含定时任务、
  上游组织引用、私有 secrets 或私有 runner。
- workflow 的 Bash 脚本语法检查通过。
- 在隔离的临时 Git 仓库执行实际 workflow 脚本：正常变更、手动执行与首次 push 均通过；
  trailing whitespace 和误提交 `dist/` 产物均被正确拒绝。
- 暂存区 `git diff --cached --check` 通过。

Rust 构建状态保持上文记录，未重复执行已知失败的 Rust 检查；依照维护者已授权的
开发快照方式提交发布。轻量 workflow 不替代应用构建与测试。
