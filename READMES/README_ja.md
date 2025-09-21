# 🎧 Razer マイク音量修正ツール

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 言語選択

[🇺🇸 English](../README.md) | [🇹🇷 Türkçe](./README_tr.md) | [🇩🇪 Deutsch](./README_de.md) | [🇫🇷 Français](./README_fr.md) | [🇪🇸 Español](./README_es.md) | [🇮🇹 Italiano](./README_it.md) | **🇯🇵 日本語** | [🇰🇷 한국어](./README_ko.md) | [🇷🇺 Русский](./README_ru.md) | [🇨🇳 中文](./README_zh.md)

---

## 🎯 このアプリケーションとは？

**Razer BlackShark V2 Pro (2023)**のマイク音量問題に対する**迅速で信頼性の高いソリューション**。このツールは、マイク音量のランダムな低下を自動的に監視・修正します。

### 🔧 問題
- Razer BlackShark V2 Pro (2023)のマイク音量がランダムに1-10%まで低下する
- ゲーム中、通話中、録音中に発生する
- ユーザーはセッションごとに何度も手動で音量を調整する必要がある

### ✅ 私たちのソリューション
- マイク音量レベルの**自動監視**
- 音量がターゲットレベルを下回った際の**リアルタイム調整**
- 中断のないバックグラウンド監視のための**システムトレイ統合**
- **多言語サポート**（10以上の言語）
- **オープンソース** - 完全に透明なコード

## 🚀 クイックスタート

### ダウンロードとインストール
1. [Releases](https://github.com/Bpolat0/razer-mic-fix/releases)ページにアクセス
2. 最新の`.exe`インストールファイルをダウンロード
3. インストーラーを実行し、セットアップウィザードに従う
4. アプリケーションを起動

### 使用方法
1. デバイスリストから**マイクを選択**
2. **ターゲット音量**を設定（推奨：85%）
3. **監視間隔**を設定（推奨：0.2秒）
4. **「開始」**をクリックして監視を開始
5. **トレイに最小化** - アプリはバックグラウンドで動作

## 🛠️ 機能

- ✅ **自動音量監視**
- ✅ **リアルタイム音量修正**
- ✅ **システムトレイ統合**
- ✅ **多言語サポート**
- ✅ **Windows起動時に開始**
- ✅ **カスタマイズ可能な設定**
- ✅ **低CPU使用率**
- ✅ **バックグラウンドサービスなし**

## 🎛️ 技術詳細

### 動作原理
1. 選択されたマイクデバイスを0.2秒ごとに**監視**（設定可能）
2. 音量がターゲットレベルを下回ったときに**検出**
3. 音量を希望レベルに**自動調整**
4. システムトレイで**静かに動作**

### システム要件
- **オペレーティングシステム**：Windows 10/11 (x64)
- **RAM**：50MB
- **CPU**：最小限の影響（< 1%）
- **権限**：管理者権限不要

## 🔒 プライバシーとセキュリティ

### このアプリが実行すること：
- ✅ オーディオデバイスレベルの監視
- ✅ 必要に応じてマイク音量の調整
- ✅ コンピューター上でのローカル設定保存
- ✅ システム通知の表示

### このアプリが実行しないこと：
- ❌ オーディオの録音や聞き取り
- ❌ マイクデータ/ストリームへのアクセス
- ❌ 外部サーバーへのデータ送信
- ❌ ドライバーやシステムサービスのインストール
- ❌ 管理者権限の要求

### オープンソース保証
- **100%オープンソース** - [全コードを表示](https://github.com/Bpolat0/razer-mic-fix)
- **テレメトリーなし**またはデータ収集なし
- **隠された機能なし**
- **コミュニティ検証済み**

## 🚨 重要な注意事項

**このソフトウェアはRazer Inc.と提携していません。**

- これは**独立したコミュニティ作成のソリューション**です
- マイクの問題を抱えるユーザーを支援するために開発されました
- **保証や補償は提供されません**
- 自己責任でご使用ください

## 🤝 貢献

貢献を歓迎します！以下の方法で支援できます：

### 🐛 バグ報告
- バグを発見しましたか？[Issue作成](https://github.com/Bpolat0/razer-mic-fix/issues)
- システム情報と再現手順を含めてください

### 💻 コード貢献
1. リポジトリをフォーク
2. 機能ブランチを作成：`git checkout -b feature/素晴らしい機能`
3. 変更をコミット：`git commit -m '素晴らしい機能を追加'`
4. ブランチをプッシュ：`git push origin feature/素晴らしい機能`
5. プルリクエストを開く

### 🌐 翻訳
アプリをより多くの言語に翻訳するのを手伝ってください：
1. `src-tauri/src/i18n/en.json`をコピー
2. あなたの言語に翻訳
3. プルリクエストを送信

## 🔨 開発セットアップ

### 前提条件
- [Rust](https://rustup.rs/)（最新安定版）
- [Node.js](https://nodejs.org/)（16+）
- [Tauri前提条件](https://tauri.app/v1/guides/getting-started/prerequisites)

### ソースからコンパイル
```bash
# リポジトリをクローン
git clone https://github.com/Bpolat0/razer-mic-fix.git
cd razer-mic-fix

# 依存関係をインストール
npm install

# 開発モード
npm run tauri dev

# プロダクション用にコンパイル
npm run tauri build
```

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細については[LICENSE](../LICENSE)ファイルを参照してください。

---

**ゲーミングコミュニティのために❤️で作成**

*最終更新：09.2025*
