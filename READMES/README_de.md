# 🎧 Razer Mikrofon Korrekturtool

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 Sprachauswahl

[🇺🇸 English](../README.md) | [🇹🇷 Türkçe](./README_tr.md) | **🇩🇪 Deutsch** | [🇫🇷 Français](./README_fr.md) | [🇪🇸 Español](./README_es.md) | [🇮🇹 Italiano](./README_it.md) | [🇯🇵 日本語](./README_ja.md) | [🇰🇷 한국어](./README_ko.md) | [🇷🇺 Русский](./README_ru.md) | [🇨🇳 中文](./README_zh.md)

---

## 🎯 Was ist diese Anwendung?

Eine **schnelle und zuverlässige Lösung** für Mikrofon-Lautstärkeprobleme beim **Razer BlackShark V2 Pro (2023)**. Dieses Tool überwacht und korrigiert automatisch zufällige Mikrofon-Lautstärkeabfälle.

### 🔧 Das Problem
- Razer BlackShark V2 Pro (2023) Mikrofon-Lautstärke fällt zufällig auf 1-10%
- Dies passiert während Gaming, Anrufen oder Aufnahmen
- Benutzer müssen die Lautstärke mehrmals pro Sitzung manuell anpassen

### ✅ Unsere Lösung
- **Automatische Überwachung** der Mikrofon-Lautstärkepegel
- **Echtzeit-Anpassung** wenn die Lautstärke unter den Zielwert fällt
- **System-Tray-Integration** für ununterbrochenes Background-Monitoring
- **Mehrsprachiger Support** (10+ Sprachen)
- **Open Source** - vollständig transparenter Code

## 🚀 Schnellstart

### Download und Installation
1. Besuchen Sie die [Releases](https://github.com/yourusername/razer-mic-fix/releases) Seite
2. Laden Sie die neueste `.exe` Installationsdatei herunter
3. Führen Sie das Installationsprogramm aus und folgen Sie dem Setup-Assistenten
4. Starten Sie die Anwendung

### Verwendung
1. **Wählen Sie Ihr Mikrofon** aus der Geräteliste
2. Stellen Sie die **Ziel-Lautstärke** ein (empfohlen: 85%)
3. Konfigurieren Sie das **Überwachungsintervall** (empfohlen: 0.2 Sekunden)
4. Klicken Sie auf **"Start"** um die Überwachung zu beginnen
5. **Minimieren in die Taskleiste** - App läuft im Hintergrund

## 🛠️ Funktionen

- ✅ **Automatische Lautstärkeüberwachung**
- ✅ **Echtzeit-Lautstärkekorrektur**
- ✅ **System-Tray-Integration**
- ✅ **Mehrsprachiger Support**
- ✅ **Start mit Windows**
- ✅ **Anpassbare Einstellungen**
- ✅ **Geringe CPU-Nutzung**
- ✅ **Keine Hintergrunddienste**

## 🎛️ Technische Details

### Funktionsweise
1. **Überwacht** das ausgewählte Mikrofon-Gerät alle 0.2 Sekunden (konfigurierbar)
2. **Erkennt** wenn die Lautstärke unter den Zielwert fällt
3. **Stellt automatisch** die Lautstärke auf das gewünschte Niveau ein
4. **Läuft leise** im System-Tray

### Systemanforderungen
- **Betriebssystem**: Windows 10/11 (x64)
- **RAM**: 50MB
- **CPU**: Minimaler Einfluss (< 1%)
- **Berechtigungen**: Keine Administratorrechte erforderlich

## 🔒 Datenschutz und Sicherheit

### Was diese App MACHT:
- ✅ Überwacht Audio-Gerätepegel
- ✅ Passt Mikrofon-Lautstärke bei Bedarf an
- ✅ Speichert Einstellungen lokal auf Ihrem Computer
- ✅ Zeigt Systembenachrichtigungen an

### Was diese App NICHT macht:
- ❌ Nimmt Audio auf oder hört zu
- ❌ Greift auf Mikrofondaten/-streams zu
- ❌ Sendet Daten an externe Server
- ❌ Installiert Treiber oder Systemdienste
- ❌ Benötigt Administratorberechtigungen

### Open-Source-Garantie
- **100% Open Source** - [vollständigen Code anzeigen](https://github.com/yourusername/razer-mic-fix)
- **Keine Telemetrie** oder Datensammlung
- **Keine versteckten Funktionen**
- **Von der Community verifiziert**

## 🚨 Wichtiger Hinweis

**Diese Software ist NICHT mit Razer Inc. verbunden.**

- Dies ist eine **unabhängige, community-erstellte Lösung**
- Entwickelt um Benutzern mit Mikrofon-Problemen zu helfen
- **Keine Garantien oder Gewährleistungen** werden gegeben
- Verwendung auf eigene Gefahr

## 🤝 Beitragen

Wir begrüßen Beiträge! So können Sie helfen:

### 🐛 Fehler melden
- Fehler gefunden? [Issue erstellen](https://github.com/yourusername/razer-mic-fix/issues)
- Systeminfo und Schritte zur Reproduktion hinzufügen

### 💻 Code-Beiträge
1. Repository forken
2. Feature-Branch erstellen: `git checkout -b feature/tolles-feature`
3. Änderungen committen: `git commit -m 'Tolles Feature hinzufügen'`
4. Branch pushen: `git push origin feature/tolles-feature`
5. Pull Request öffnen

### 🌐 Übersetzungen
Helfen Sie uns, die App in mehr Sprachen zu übersetzen:
1. Kopieren Sie `src-tauri/src/i18n/en.json`
2. In Ihre Sprache übersetzen
3. Pull Request einreichen

## 🔨 Entwicklungssetup

### Voraussetzungen
- [Rust](https://rustup.rs/) (neueste stabile Version)
- [Node.js](https://nodejs.org/) (16+)
- [Tauri-Voraussetzungen](https://tauri.app/v1/guides/getting-started/prerequisites)

### Aus Quellcode kompilieren
```bash
# Repository klonen
git clone https://github.com/yourusername/razer-mic-fix.git
cd razer-mic-fix

# Abhängigkeiten installieren
npm install

# Entwicklungsmodus
npm run tauri dev

# Für Produktion kompilieren
npm run tauri build
```

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](../LICENSE) Datei für Details.

---

**Mit ❤️ für die Gaming-Community erstellt**

*Zuletzt aktualisiert: 09.2025*
