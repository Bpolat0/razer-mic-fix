# 🎧 Razer Mic Volume Fixer

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 Language / Dil Seçimi

| Language | Dil | README |
|----------|-----|--------|
| 🇺🇸 | **English** | **Current** |
| 🇹🇷 | **Türkçe** | [Turkish](./READMES/README_tr.md) |
| 🇩🇪 | **Deutsch** | [German](./READMES/README_de.md) |
| 🇫🇷 | **Français** | [French](./READMES/README_fr.md) |
| 🇪🇸 | **Español** | [Spanish](./READMES/README_es.md) |
| 🇮🇹 | **Italiano** | [Italian](./READMES/README_it.md) |
| 🇯🇵 | **日本語** | [Japanese](./READMES/README_ja.md) |
| 🇰🇷 | **한국어** | [Korean](./READMES/README_ko.md) |
| 🇷🇺 | **Русский** | [Russian](./READMES/README_ru.md) |
| 🇨🇳 | **中文** | [Chinese](./READMES/README_zh.md) |

---

## 🎯 What is this?

A **fast and reliable solution** for the **Razer BlackShark V2 Pro (2023)** microphone volume issue. This tool automatically monitors and fixes the microphone volume when it randomly drops to low levels.

### 🔧 The Problem
- Razer BlackShark V2 Pro (2023) microphone volume randomly drops to 1-10%
- This happens during gaming, calls, or recording
- Users have to manually adjust the volume multiple times per session

### ✅ Our Solution
- **Automatic monitoring** of microphone volume levels
- **Real-time adjustment** when volume drops below target
- **System tray integration** for seamless background operation
- **Multi-language support** (10+ languages)
- **Open source** - completely transparent code

## 🚀 Quick Start

### Download & Run (No Installation!)
1. Go to [Releases](https://github.com/Bpolat0/razer-mic-fix/releases)
2. Download the latest `RazerMicFixer.exe` file
3. **Double-click to run immediately** - No installation required!
4. Place the file anywhere you want on your computer

### Usage
1. **Select your microphone** from the device dropdown
2. **Set target volume** (recommended: 85%)
3. **Set check interval** (recommended: 0.2 seconds)
4. Click **"Start"** to begin monitoring
5. **Minimize to tray** - the app works in the background

## 🛠️ Features

- ✅ **Automatic Volume Monitoring**
- ✅ **Real-time Volume Correction**
- ✅ **System Tray Integration**
- ✅ **Multi-language Support**
- ✅ **Start with Windows**
- ✅ **Customizable Settings**
- ✅ **Low CPU Usage**
- ✅ **No Background Services**

## 🎛️ Technical Details

### How It Works
1. **Monitors** your selected microphone device every 0.2 seconds (configurable)
2. **Detects** when volume drops below your target level
3. **Automatically adjusts** the volume back to your desired level
4. **Runs silently** in the system tray

### System Requirements
- **OS**: Windows 10/11 (x64)
- **RAM**: 50MB
- **CPU**: Minimal impact (< 1%)
- **Permissions**: No admin rights required

## 🔒 Privacy & Security

### What This App Does:
- ✅ Monitors audio device volume levels
- ✅ Adjusts microphone volume when needed
- ✅ Saves settings locally on your computer
- ✅ Shows system notifications

### What This App Does NOT Do:
- ❌ Record or listen to audio
- ❌ Access microphone data/streams
- ❌ Send data to external servers
- ❌ Install drivers or system services
- ❌ Require admin privileges

### Open Source Guarantee
- **100% open source** - [view all code](https://github.com/Bpolat0/razer-mic-fix)
- **No telemetry** or data collection
- **No hidden functionality**
- **Community verified**

## 🚨 Important Disclaimer

**This software is NOT affiliated with Razer Inc.**

- This is an **independent, community-made solution**
- Created to help users experiencing microphone issues
- **No warranty or guarantee** provided
- Use at your own discretion

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### 🐛 Report Issues
- Found a bug? [Create an issue](https://github.com/Bpolat0/razer-mic-fix/issues)
- Include your system info and steps to reproduce

### 💻 Code Contributions
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### 🌐 Translations
Help us translate the app into more languages:
1. Copy `src-tauri/src/i18n/en.json`
2. Translate to your language
3. Submit a Pull Request

### 💰 Support
If this tool helped you:
- ⭐ **Star the repository**
- 🐛 **Report issues** you find
- 📢 **Share with others** who need it
- 💻 **Contribute code** improvements

## 🔨 Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (16+)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Build from Source
```bash
# Clone the repository
git clone https://github.com/Bpolat0/razer-mic-fix.git
cd razer-mic-fix

# Install dependencies
npm install

# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

- **Tauri Team** - For the amazing framework
- **Rust Community** - For the powerful language
- **Users** - For testing and feedback
- **Contributors** - For improvements and translations

---

**Made with ❤️ for the gaming community**

*Last updated: September 2025*
