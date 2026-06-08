# 参与贡献

感谢参与 LanMind / 澜灵。

## 本地开发

需要 Node.js 24、pnpm 11、Rust stable、Windows WebView2 与 MSVC Build Tools。

```powershell
pnpm install
pnpm build
cargo check -p lanmind-desktop
pnpm tauri dev
```

## 原则

- 权限透明，敏感能力默认关闭。
- 危险操作必须经过明确确认。
- 默认低占用，不持续调用模型，不扫描全盘。
- 保持离线可用，自定义 Provider 不依赖澜算云。

提交 Pull Request 前请确保前端构建与 Rust 检查通过。
