# DUTools

## 配置开发环境

要开发此项目，需要具备以下环境：

- Node.js with pnpm
- Rust

具体请参阅 [前置要求 | Tauri](https://tauri.app/zh-cn/start/prerequisites/)。

此外，项目使用 pnet 获取用户网卡信息。在 Windows 上，需要安装 WinPcap 或者 Npcap，具体请见 [libpnet usage on Windows](https://github.com/libpnet/libpnet?tab=readme-ov-file#windows)。

之后，执行以下命令安装依赖：

```bash
pnpm i
```

最后，执行以下命令以开发模式启动项目：

```bash
pnpm dev
```
