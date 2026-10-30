# 参与 Warp Lite 开发

Issue 和 PR 均提交到 [AnxiangLemon/warp-lite](https://github.com/AnxiangLemon/warp-lite)。本项目独立维护，不使用上游的 Oz 审核、团队指派、同步或发布流程。

## 提交问题

Bug 请提供版本或 commit、操作系统、shell、复现步骤和预期结果，必要时附截图或相关日志。功能建议请说明使用场景与预期行为。请先搜索本仓库已有 Issue。

## 提交代码

从 `main` 创建分支，保持改动集中，并使用仓库的 PR 模板说明行为变化与验证结果。构建和检查命令见 [开发说明](docs/development.md)，编码约定见 [AGENTS.md](AGENTS.md)。

Rust 改动应执行对应的格式化、Clippy 与测试；GUI 行为需要实际运行验证。文档和 workflow 改动检查文件格式、链接、脚本与配置语法。现有源码的已知构建问题见 [验证记录](docs/validation.md)，请如实记录受阻或未执行的检查。

`Repository checks` 只检查 whitespace 和误提交的本地产物，它通过不代表应用已构建通过。

不要提交 `target*/`、`dist/`、本地配置或排错日志。保留编译所需资源、许可证和 `Cargo.lock`。本项目不自动拉取或合并上游更新。
