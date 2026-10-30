# Warp Lite 自动化

仅保留 `repository-checks.yml`：在 `main` 的 push、PR 或手动触发时检查变更行的
whitespace，以及是否误提交构建产物、排错日志和本地配置。
同时用 Python 标准库核对本地依赖源码 SHA256，并检查 lockfile 没有 Git 来源。
vendor 保留原始字节与 whitespace，完整性由来源清单校验。

使用一个 GitHub-hosted Linux runner、只读仓库权限和固定版本的官方 checkout action。
不安装 Rust 或下载 LFS 模型，不编译应用，也不需要维护者配置 secrets。

已移除上游同步、发布、缓存预热、容器镜像、Slack / 文档通知、Oz 机器人、
自动分派审核与 skill 更新等 workflows，以及它们专用的 actions、脚本和发布配置。
Dependabot 的上游私有仓库凭据与团队指派配置也已移除。

这个检查通过只表示仓库检查通过；构建状态以 `docs/validation.md` 和本地构建结果为准。
后续需要正式构建或发布时，应为 Lite 单独添加流程，不复用上游私有基础设施。
