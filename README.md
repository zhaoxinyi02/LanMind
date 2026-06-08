# LanMind / 澜灵

> Windows 优先、权限透明、离线可用的主动式电脑 AI 管家。

[![CI](https://github.com/zhaoxinyi02/LanMind/actions/workflows/ci.yml/badge.svg)](https://github.com/zhaoxinyi02/LanMind/actions/workflows/ci.yml)
[![Release](https://github.com/zhaoxinyi02/LanMind/actions/workflows/release.yml/badge.svg)](https://github.com/zhaoxinyi02/LanMind/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

澜灵不是一个只能等待指令的聊天机器人。它将通过桌面宠物与多页面控制台观察电脑状态、主动发现值得关注的问题，并在用户授权后执行操作。

项目当前处于 **MVP 0.1 早期预览阶段**。

## 当前能力

- Vue 3 + TypeScript 控制台：首页、模型服务、权限中心、账号入口、设置
- 自定义 OpenAI-compatible Provider 本地配置
- 权限枚举与默认安全策略
- Tauri 2 桌面壳、系统托盘、桌宠窗口配置
- SQLite 初始化迁移

## 下载

Windows 安装包由 GitHub Actions 自动构建，请前往 [Releases](https://github.com/zhaoxinyi02/LanMind/releases) 下载最新预览版。

## 开发

要求：Node.js 24、pnpm 11、Rust stable、Windows WebView2 与 MSVC Build Tools。

```powershell
pnpm install
pnpm dev
pnpm tauri dev
```

危险命令执行、文件移动和敏感数据读取在 MVP 0.1 中均未实现。

## 项目结构

- `apps/desktop`：Tauri 2 桌面应用
- `packages/shared`：公共数据类型
- `packages/model-router`：模型 Provider 路由基础
- `packages/permission`：权限决策基础
- `docs`：产品与架构文档

## 路线图

- `0.1`：桌面骨架、首页、桌宠、模型配置、权限中心
- `0.2`：文件监听、主动发现、整理预览与通知
- `0.3`：终端报错分析、风险评级与确认执行
- `0.4`：项目理解、Git 状态与下一步建议

## 贡献与安全

参与开发前请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。安全问题请按照 [SECURITY.md](SECURITY.md) 私下报告。
