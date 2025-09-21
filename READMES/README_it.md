# 🎧 Correttore Microfono Razer

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 Selezione Lingua

[🇺🇸 English](../README.md) | [🇹🇷 Türkçe](./README_tr.md) | [🇩🇪 Deutsch](./README_de.md) | [🇫🇷 Français](./README_fr.md) | [🇪🇸 Español](./README_es.md) | **🇮🇹 Italiano** | [🇯🇵 日本語](./README_ja.md) | [🇰🇷 한국어](./README_ko.md) | [🇷🇺 Русский](./README_ru.md) | [🇨🇳 中文](./README_zh.md)

---

## 🎯 Cos'è questa Applicazione?

Una **soluzione rapida e affidabile** per i problemi di volume del microfono **Razer BlackShark V2 Pro (2023)**. Questo strumento monitora e corregge automaticamente i cali casuali del volume del microfono.

### 🔧 Il Problema
- Il volume del microfono Razer BlackShark V2 Pro (2023) scende casualmente all'1-10%
- Questo accade durante il gaming, le chiamate o le registrazioni
- Gli utenti devono regolare manualmente il volume più volte per sessione

### ✅ La Nostra Soluzione
- **Monitoraggio automatico** dei livelli di volume del microfono
- **Regolazione in tempo reale** quando il volume scende sotto il livello target
- **Integrazione nella system tray** per monitoraggio continuo in background
- **Supporto multilingue** (10+ lingue)
- **Open Source** - codice completamente trasparente

## 🚀 Avvio Rapido

### Download e Installazione
1. Visita la pagina [Releases](https://github.com/yourusername/razer-mic-fix/releases)
2. Scarica l'ultimo file di installazione `.exe`
3. Esegui l'installer e segui la procedura guidata di configurazione
4. Avvia l'applicazione

### Utilizzo
1. **Seleziona il tuo microfono** dalla lista dei dispositivi
2. Imposta il **volume target** (raccomandato: 85%)
3. Configura l'**intervallo di monitoraggio** (raccomandato: 0.2 secondi)
4. Clicca su **"Avvia"** per iniziare il monitoraggio
5. **Minimizza nella tray** - l'app funziona in background

## 🛠️ Caratteristiche

- ✅ **Monitoraggio Automatico del Volume**
- ✅ **Correzione Volume in Tempo Reale**
- ✅ **Integrazione System Tray**
- ✅ **Supporto Multilingue**
- ✅ **Avvio con Windows**
- ✅ **Impostazioni Personalizzabili**
- ✅ **Basso Utilizzo CPU**
- ✅ **Nessun Servizio in Background**

## 🎛️ Dettagli Tecnici

### Come Funziona
1. **Monitora** il dispositivo microfono selezionato ogni 0.2 secondi (configurabile)
2. **Rileva** quando il volume scende sotto il livello target
3. **Regola automaticamente** il volume al livello desiderato
4. **Funziona silenziosamente** nella system tray

### Requisiti di Sistema
- **Sistema Operativo**: Windows 10/11 (x64)
- **RAM**: 50MB
- **CPU**: Impatto minimo (< 1%)
- **Permessi**: Nessun diritto di amministratore richiesto

## 🔒 Privacy e Sicurezza

### Cosa FA questa App:
- ✅ Monitora i livelli dei dispositivi audio
- ✅ Regola il volume del microfono quando necessario
- ✅ Memorizza le impostazioni localmente sul tuo computer
- ✅ Mostra notifiche di sistema

### Cosa NON FA questa App:
- ❌ Non registra o ascolta audio
- ❌ Non accede a dati/stream del microfono
- ❌ Non invia dati a server esterni
- ❌ Non installa driver o servizi di sistema
- ❌ Non richiede permessi di amministratore

### Garanzia Open Source
- **100% Open Source** - [visualizza tutto il codice](https://github.com/yourusername/razer-mic-fix)
- **Nessuna telemetria** o raccolta dati
- **Nessuna funzionalità nascosta**
- **Verificato dalla community**

## 🚨 Avviso Importante

**Questo software NON è affiliato con Razer Inc.**

- Questa è una **soluzione indipendente creata dalla community**
- Sviluppata per aiutare gli utenti con problemi del microfono
- **Nessuna garanzia o assicurazione** è fornita
- Usa a tuo rischio

## 🤝 Contribuire

Accogliamo i contributi! Ecco come puoi aiutare:

### 🐛 Segnalare Bug
- Trovato un bug? [Crea un issue](https://github.com/yourusername/razer-mic-fix/issues)
- Includi informazioni di sistema e passaggi per riprodurre

### 💻 Contributi al Codice
1. Forka il repository
2. Crea un branch per la feature: `git checkout -b feature/feature-fantastica`
3. Committa le modifiche: `git commit -m 'Aggiungi feature fantastica'`
4. Pusha il branch: `git push origin feature/feature-fantastica`
5. Apri una Pull Request

### 🌐 Traduzioni
Aiutaci a tradurre l'app in più lingue:
1. Copia `src-tauri/src/i18n/en.json`
2. Traduci nella tua lingua
3. Invia una Pull Request

## 🔨 Setup di Sviluppo

### Prerequisiti
- [Rust](https://rustup.rs/) (ultima versione stabile)
- [Node.js](https://nodejs.org/) (16+)
- [Prerequisiti Tauri](https://tauri.app/v1/guides/getting-started/prerequisites)

### Compilare dal Codice Sorgente
```bash
# Clona il repository
git clone https://github.com/yourusername/razer-mic-fix.git
cd razer-mic-fix

# Installa le dipendenze
npm install

# Modalità sviluppo
npm run tauri dev

# Compila per produzione
npm run tauri build
```

## 📄 Licenza

Questo progetto è sotto licenza MIT - vedi il file [LICENSE](../LICENSE) per i dettagli.

---

**Creato con ❤️ per la gaming community**

*Ultimo aggiornamento: 09.2025*
