# Lite 独立维护与源码依赖

本仓库独立维护本地 GUI 终端。没有自动同步上游的 workflow，不需要上游账户、私有
Git 仓库凭据或 gcloud 登录；终端中用户自己运行的工具仍由其自身决定网络行为。

本轮同时纳入现有的 Lite 图标、字体和图形内存优化、中文设置、WSL 目录搜索及退出时
查看进程的焦点修复。此前单项实验记录保留在 `validation.md`，本轮合并后的结果单独记录。

## 本地源码

现在保留 34 个本地源码 package，放在 `vendor/upstream/`；workflow 数据改为固定的
生成结果，删除了构建时 generator 和
`uneval` 依赖。原来的字体、窗口、终端解析等补丁继续保留，避免未经验证地切换实现。

`vendor/sources.json` 记录每个 package 的版本、原始仓库和 commit、许可证、本地修改
及逐文件 SHA256；各目录保留原许可证、版权声明和原始规范化 manifest。原仓库 URL
用于溯源，Cargo 使用本地 path，不会因为这些 URL 自动拉取或合并更新。

普通 crates.io 依赖仍由 `Cargo.lock` 固定。首次构建需要下载 registry 依赖，或使用已有
本地缓存；这里不承诺一台没有任何缓存的新机器能够断网构建。必要的原生工具链仍需
安装。当前资源无需 Git LFS。

本地依赖中不参与应用构建的上游 LFS 测试字体与截图占位文件已省略，不需要为这些
测试资源连接上游 LFS 服务。保留库的运行源码与许可证，并在来源清单记录裁剪内容。

未在 Lite 启用的 jemalloc Git override 已删除；遗留可选 profiling feature 使用
crates.io 的 allocator package。本轮不验证该可选配置。

```powershell
python .\script\verify-local-sources.py
.\script\verify-lite-dependencies.ps1
cargo check -p warp --bin warp-lite --no-default-features --features warp_lite --frozen
```

修改本地 vendor 后，应检查差异，补充 `local_changes` 并重新计算对应文件的 SHA256。
`.gitattributes` 禁止转换 vendor 文件换行，以便 Windows 和 Linux 校验相同的字节。

## 删除和保留的边界

- 删除独立 `warp_tui` 前端、`serve-wasm` 开发服务器、旧功能规划和调试 PDB。
- 删除 ONNX 模型、模型评估工具及 Candle / ORT / tokenizer 依赖，使用本地 heuristic。
- 把 WebSocket 连接和 HTTP 的默认 TLS 依赖放入显式 transport feature；Lite 禁用它们。
- Windows x64 Lite 运行依赖从 652 降到 639 个 package/version 组合，不包含 TLS /
  AWS-LC、ONNX / Candle、AWS SDK、Tantivy、Tink、libgit2 或 QuickJS。
- 开发入口不再下载上游 skills、不安装或登录 gcloud。`script/run` 默认运行 Lite。
- 保留终端 / GPU / 字体 / SQLite、会话与持久化类型、共享模块及其兼容接口。
  部分旧功能源码仍供共享类型和完整配置的检查使用，不表示 Lite 启用了这些服务。

本轮删除约 134 MiB 文件，加入约 44 MiB 必要本地源码，当前源码树净减少约 90 MiB。
这是文件内容大小，不是 EXE 大小或 Git 历史压缩包大小。旧 Git 对象不会因删除当前
文件而自动消失。

## Git 历史

保留历史有助于追溯修改和源码来源，不会让上游影响当前代码。GitHub 的历史贡献者
列表来自 commit 记录，不能当作当前维护者或当前功能贡献的清单。删除功能也不改变
保留下来的派生代码的来源、许可证和版权声明。

当前没有技术上的重建历史要求。仅需最新 Lite 代码时，可使用浅克隆：

```powershell
git clone --depth 1 https://github.com/AnxiangLemon/warp-lite.git
```

如以后决定从一个新的 root commit 开始，应先备份旧历史及所需 LFS 对象，再单独安排
远端历史切换；本轮不重写历史，不强制推送，也不改写原作者身份。

## 验证范围

本轮用仅映射 registry 缓存、没有 `git/` cache 的独立 `CARGO_HOME`，通过 `--frozen`
检查和构建。这验证必要的上游 Git checkout 已不再是构建前提。

本轮结果见下表。验证平台为 Windows x64 / MSVC / Rust 1.92.0。
macOS、Linux 和 Windows ARM64 未在本轮构建运行；本机无 `oz-dev`，无法发现云端 runner。


## 本轮验证记录（2026-10-31）

本轮包含依赖本地化和 02:57 固定的 56 文件修复快照（含 JPEG / 异步背景与启动优化）。测试、优化构建与打包均来自合并后的源码；
未使用旧 `dist/` 程序替代当前源码验证。

| 检查 | 结果 |
| --- | --- |
| `script/format`、`--check`、inline test module 检查 | 通过 |
| workspace / `warp` / `warp_completer` 三组严格 Clippy | 全部通过，使用 `script/presubmit` 配置及 `-D warnings` |
| Lite 应用、会话、背景、遥测、SQLite、目录处理 | 62 项通过 |
| 字体排版与图像 / SVG 缓存 | 29 项通过 |
| 离线服务配置 | 45 项通过 |
| heuristic classifier 与禁用 WebSocket | 6 项通过 |
| 本地源码清单、无 Git cache 的 Cargo 解析及构建 | 通过，34 个本地源码 package，零 Git 来源 |
| PowerShell / Bash / Python 语法、文档链接、暂存区 whitespace / 产物排除 | 通过 |
| 优化构建、Windows 本地打包、EXE `--version` | 通过 |

142 项是针对性单元测试总数，不是完整 workspace test suite。本机没有 `cargo-nextest`，
使用 `cargo test` 及其生成的测试程序。Lite 构建仍有共享模块的 `dead_code` 警告，未全局
关闭 lint；三组严格 Clippy 与 Lite 专用 feature 配置不是同一个配置。

真实显示的 GUI 用例使用独立 `WarpIntegrationLiteSmoke-*` 应用与数据目录：

| 用例 | 结果 |
| --- | --- |
| `test_lite_terminal_smoke` | 通过；中文路径、命令执行 / 取消、持久化与真实截图 |
| `test_lite_terminal_management` | 通过；标签、分组、分屏 |
| `test_lite_chinese_settings` | 通过；中文搜索、主题、Lite 图标和信息页 |
| `test_lite_restore_named_tabs` | 通过 |
| `test_lite_restore_saved_tabs` | 通过；SQLite backup API 的一致性快照 |
| `test_lite_close_confirmation` | 通过；终端 / 设置页查看进程、焦点和取消退出 |
| `test_lite_wsl_directory_search` | 最新隔离配置通过 |
| PNG / JPEG / 缺失 / 损坏 / 超限背景 5 项 GUI 用例 | 全部通过，包含异步加载与主题切换 |
| 显式 `WGPU_BACKEND=vulkan` 的 smoke test | 通过 |

前一轮 WSL 用例曾在首次运行时混入 Bash 输入同步控制序列 `^[i^P`，同一源码随后
连续复跑 3 次通过。最新快照将测试的 `WSLENV` 设为 `HOME/p`，让 WSL 使用生成的隔离
rcfiles，避免依赖用户的 Linux 启动配置；本轮完整 GUI 回归中该用例首轮通过。没有专门
重复制造 WSL 服务冷启动来验证因果关系，以上记录不等同于冷启动稳定性压测。

字体 / 图像缓存与离线服务等未变动组件使用本轮此前通过的 80 项单元测试结果；最新
应用源码重新通过 62 项针对性单元测试。二者总计 142 项，均对应当前相同的组件源码。


本地优化 EXE 为 **53,260,800 字节（50.79 MiB）**，产品名和描述
均为 `Warp Lite`，`--version` 输出 `Warp Lite 0.1.0`，退出码 0。

SHA256：`C907B5E2F718A8D49A4068719F1E356310B8CD4840C844A80D15C96C9C53C942`。

验证包位于被忽略的 `dist/Lite Independence Snapshot/`，不提交二进制。包内保留单项
许可证，但还没有完整 Cargo 第三方依赖许可清单，因此本轮仅发布源码，不作为正式二进制
发行包。日志、依赖 JSON、截图与原始未提交文件备份位于被忽略的 `.note/independence/`。
