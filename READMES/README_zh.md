# 🎧 雷蛇麦克风音量修复工具

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 语言选择

[🇺🇸 English](../README.md) | [🇹🇷 Türkçe](./README_tr.md) | [🇩🇪 Deutsch](./README_de.md) | [🇫🇷 Français](./README_fr.md) | [🇪🇸 Español](./README_es.md) | [🇮🇹 Italiano](./README_it.md) | [🇯🇵 日本語](./README_ja.md) | [🇰🇷 한국어](./README_ko.md) | [🇷🇺 Русский](./README_ru.md) | **🇨🇳 中文**

---

## 🎯 这个应用程序是什么？

为 **雷蛇 BlackShark V2 Pro (2023)** 麦克风音量问题提供的 **快速可靠的解决方案**。此工具自动监控并修复麦克风音量的随机下降。

### 🔧 问题
- 雷蛇 BlackShark V2 Pro (2023) 麦克风音量随机下降到 1-10%
- 这在游戏、通话或录音期间发生
- 用户必须在每次会话中多次手动调整音量

### ✅ 我们的解决方案
- **自动监控** 麦克风音量级别
- 当音量低于目标级别时进行 **实时调整**
- **系统托盘集成** 实现不间断的后台监控
- **多语言支持**（10+ 种语言）
- **开源** - 完全透明的代码

## 🚀 快速开始

### 下载和安装
1. 访问 [Releases](https://github.com/yourusername/razer-mic-fix/releases) 页面
2. 下载最新的 `.exe` 安装文件
3. 运行安装程序并按照设置向导操作
4. 启动应用程序

### 使用方法
1. 从设备列表中 **选择您的麦克风**
2. 设置 **目标音量**（推荐：85%）
3. 配置 **监控间隔**（推荐：0.2 秒）
4. 点击 **"开始"** 开始监控
5. **最小化到托盘** - 应用程序在后台运行

## 🛠️ 功能特性

- ✅ **自动音量监控**
- ✅ **实时音量修正**
- ✅ **系统托盘集成**
- ✅ **多语言支持**
- ✅ **随 Windows 启动**
- ✅ **可自定义设置**
- ✅ **低 CPU 使用率**
- ✅ **无后台服务**

## 🎛️ 技术详情

### 工作原理
1. 每 0.2 秒 **监控** 选定的麦克风设备（可配置）
2. **检测** 音量低于目标级别的情况
3. **自动调整** 音量到所需级别
4. 在系统托盘中 **静默运行**

### 系统要求
- **操作系统**：Windows 10/11 (x64)
- **内存**：50MB
- **CPU**：最小影响（< 1%）
- **权限**：无需管理员权限

## 🔒 隐私与安全

### 此应用程序会做什么：
- ✅ 监控音频设备级别
- ✅ 必要时调整麦克风音量
- ✅ 在您的计算机上本地存储设置
- ✅ 显示系统通知

### 此应用程序不会做什么：
- ❌ 不录制或监听音频
- ❌ 不访问麦克风数据/流
- ❌ 不向外部服务器发送数据
- ❌ 不安装驱动程序或系统服务
- ❌ 不需要管理员权限

### 开源保证
- **100% 开源** - [查看所有代码](https://github.com/yourusername/razer-mic-fix)
- **无遥测** 或数据收集
- **无隐藏功能**
- **社区验证**

## 🚨 重要声明

**此软件与雷蛇公司无关。**

- 这是一个 **独立的社区创建解决方案**
- 开发用于帮助有麦克风问题的用户
- **不提供任何保证或担保**
- 使用风险自负

## 🤝 贡献

我们欢迎贡献！以下是您可以提供帮助的方式：

### 🐛 报告错误
- 发现错误？[创建 issue](https://github.com/yourusername/razer-mic-fix/issues)
- 包含系统信息和重现步骤

### 💻 代码贡献
1. Fork 仓库
2. 创建功能分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 开启 Pull Request

### 🌐 翻译
帮助我们将应用程序翻译成更多语言：
1. 复制 `src-tauri/src/i18n/en.json`
2. 翻译成您的语言
3. 提交 Pull Request

## 🔨 开发设置

### 先决条件
- [Rust](https://rustup.rs/)（最新稳定版）
- [Node.js](https://nodejs.org/)（16+）
- [Tauri 先决条件](https://tauri.app/v1/guides/getting-started/prerequisites)

### 从源代码编译
```bash
# 克隆仓库
git clone https://github.com/yourusername/razer-mic-fix.git
cd razer-mic-fix

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 为生产编译
npm run tauri build
```

## 📄 许可证

此项目在 MIT 许可证下授权 - 有关详细信息，请参阅 [LICENSE](../LICENSE) 文件。

---

**为游戏社区用 ❤️ 制作**

*最后更新：09.2025*
