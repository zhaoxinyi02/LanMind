# 本地架构

Vue 控制台通过 Tauri IPC 调用 Rust 本地核心。Rust 负责桌面壳、系统边界和 SQLite；TypeScript 负责 Provider、权限与未来主动引擎的上层逻辑。

所有操作能力必须先经过权限判断。禁止能力不可被配置为自动执行。
