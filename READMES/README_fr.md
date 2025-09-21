# 🎧 Correcteur de Microphone Razer

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 Sélection de Langue

[🇺🇸 English](../README.md) | [🇹🇷 Türkçe](./README_tr.md) | [🇩🇪 Deutsch](./README_de.md) | **🇫🇷 Français** | [🇪🇸 Español](./README_es.md) | [🇮🇹 Italiano](./README_it.md) | [🇯🇵 日本語](./README_ja.md) | [🇰🇷 한국어](./README_ko.md) | [🇷🇺 Русский](./README_ru.md) | [🇨🇳 中文](./README_zh.md)

---

## 🎯 Qu'est-ce que cette Application ?

Une **solution rapide et fiable** pour les problèmes de volume du microphone **Razer BlackShark V2 Pro (2023)**. Cet outil surveille et corrige automatiquement les chutes aléatoires du volume du microphone.

### 🔧 Le Problème
- Le volume du microphone Razer BlackShark V2 Pro (2023) chute aléatoirement à 1-10%
- Cela se produit pendant les jeux, les appels ou les enregistrements
- Les utilisateurs doivent ajuster manuellement le volume plusieurs fois par session

### ✅ Notre Solution
- **Surveillance automatique** des niveaux de volume du microphone
- **Ajustement en temps réel** quand le volume descend sous le niveau cible
- **Intégration de la barre d'état système** pour une surveillance en arrière-plan ininterrompue
- **Support multilingue** (10+ langues)
- **Open Source** - code entièrement transparent

## 🚀 Démarrage Rapide

### Téléchargement et Installation
1. Visitez la page [Releases](https://github.com/yourusername/razer-mic-fix/releases)
2. Téléchargez le dernier fichier d'installation `.exe`
3. Exécutez le programme d'installation et suivez l'assistant de configuration
4. Lancez l'application

### Utilisation
1. **Sélectionnez votre microphone** dans la liste des appareils
2. Définissez le **volume cible** (recommandé : 85%)
3. Configurez l'**intervalle de surveillance** (recommandé : 0,2 secondes)
4. Cliquez sur **"Démarrer"** pour commencer la surveillance
5. **Réduire dans la barre d'état** - l'app fonctionne en arrière-plan

## 🛠️ Fonctionnalités

- ✅ **Surveillance Automatique du Volume**
- ✅ **Correction du Volume en Temps Réel**
- ✅ **Intégration de la Barre d'État Système**
- ✅ **Support Multilingue**
- ✅ **Démarrage avec Windows**
- ✅ **Paramètres Personnalisables**
- ✅ **Faible Utilisation CPU**
- ✅ **Aucun Service en Arrière-plan**

## 🎛️ Détails Techniques

### Comment ça Marche
1. **Surveille** l'appareil microphone sélectionné toutes les 0,2 secondes (configurable)
2. **Détecte** quand le volume descend sous le niveau cible
3. **Ajuste automatiquement** le volume au niveau désiré
4. **Fonctionne silencieusement** dans la barre d'état système

### Configuration Système Requise
- **Système d'exploitation** : Windows 10/11 (x64)
- **RAM** : 50MB
- **CPU** : Impact minimal (< 1%)
- **Permissions** : Aucun droit d'administrateur requis

## 🔒 Confidentialité et Sécurité

### Ce que cette App FAIT :
- ✅ Surveille les niveaux des appareils audio
- ✅ Ajuste le volume du microphone quand nécessaire
- ✅ Stocke les paramètres localement sur votre ordinateur
- ✅ Affiche les notifications système

### Ce que cette App NE FAIT PAS :
- ❌ N'enregistre pas ou n'écoute pas l'audio
- ❌ N'accède pas aux données/flux du microphone
- ❌ N'envoie pas de données à des serveurs externes
- ❌ N'installe pas de pilotes ou services système
- ❌ Ne nécessite pas de permissions d'administrateur

### Garantie Open Source
- **100% Open Source** - [voir tout le code](https://github.com/yourusername/razer-mic-fix)
- **Aucune télémétrie** ou collecte de données
- **Aucune fonctionnalité cachée**
- **Vérifié par la communauté**

## 🚨 Avertissement Important

**Ce logiciel N'EST PAS affilié à Razer Inc.**

- Il s'agit d'une **solution indépendante créée par la communauté**
- Développé pour aider les utilisateurs ayant des problèmes de microphone
- **Aucune garantie ou assurance** n'est fournie
- Utilisation à vos propres risques

## 🤝 Contribuer

Nous accueillons les contributions ! Voici comment vous pouvez aider :

### 🐛 Signaler des Bugs
- Trouvé un bug ? [Créer une issue](https://github.com/yourusername/razer-mic-fix/issues)
- Inclure les infos système et les étapes de reproduction

### 💻 Contributions de Code
1. Fork le dépôt
2. Créer une branche de fonctionnalité : `git checkout -b feature/fonctionnalite-geniale`
3. Commit les changements : `git commit -m 'Ajouter fonctionnalité géniale'`
4. Push la branche : `git push origin feature/fonctionnalite-geniale`
5. Ouvrir une Pull Request

### 🌐 Traductions
Aidez-nous à traduire l'app dans plus de langues :
1. Copier `src-tauri/src/i18n/en.json`
2. Traduire dans votre langue
3. Soumettre une Pull Request

## 🔨 Configuration de Développement

### Prérequis
- [Rust](https://rustup.rs/) (dernière version stable)
- [Node.js](https://nodejs.org/) (16+)
- [Prérequis Tauri](https://tauri.app/v1/guides/getting-started/prerequisites)

### Compiler depuis les Sources
```bash
# Cloner le dépôt
git clone https://github.com/yourusername/razer-mic-fix.git
cd razer-mic-fix

# Installer les dépendances
npm install

# Mode développement
npm run tauri dev

# Compiler pour la production
npm run tauri build
```

## 📄 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](../LICENSE) pour les détails.

---

**Créé avec ❤️ pour la communauté gaming**

*Dernière mise à jour : 09.2025*
