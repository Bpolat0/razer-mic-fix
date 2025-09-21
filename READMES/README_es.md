# 🎧 Corrector de Micrófono Razer

[![GitHub Release](https://img.shields.io/github/v/release/Bpolat0/razer-mic-fix)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/Bpolat0/razer-mic-fix/total)](https://github.com/Bpolat0/razer-mic-fix/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🌐 Selección de Idioma

[🇺🇸 English](../README.md) | [🇹🇷 Türkçe](./README_tr.md) | [🇩🇪 Deutsch](./README_de.md) | [🇫🇷 Français](./README_fr.md) | **🇪🇸 Español** | [🇮🇹 Italiano](./README_it.md) | [🇯🇵 日本語](./README_ja.md) | [🇰🇷 한국어](./README_ko.md) | [🇷🇺 Русский](./README_ru.md) | [🇨🇳 中文](./README_zh.md)

---

## 🎯 ¿Qué es esta Aplicación?

Una **solución rápida y confiable** para problemas de volumen del micrófono **Razer BlackShark V2 Pro (2023)**. Esta herramienta monitorea y corrige automáticamente las caídas aleatorias del volumen del micrófono.

### 🔧 El Problema
- El volumen del micrófono Razer BlackShark V2 Pro (2023) cae aleatoriamente al 1-10%
- Esto sucede durante juegos, llamadas o grabaciones
- Los usuarios deben ajustar manualmente el volumen varias veces por sesión

### ✅ Nuestra Solución
- **Monitoreo automático** de los niveles de volumen del micrófono
- **Ajuste en tiempo real** cuando el volumen cae por debajo del nivel objetivo
- **Integración en la bandeja del sistema** para monitoreo continuo en segundo plano
- **Soporte multiidioma** (10+ idiomas)
- **Código abierto** - código completamente transparente

## 🚀 Inicio Rápido

### Descarga e Instalación
1. Visita la página de [Releases](https://github.com/yourusername/razer-mic-fix/releases)
2. Descarga el último archivo de instalación `.exe`
3. Ejecuta el instalador y sigue el asistente de configuración
4. Inicia la aplicación

### Uso
1. **Selecciona tu micrófono** de la lista de dispositivos
2. Establece el **volumen objetivo** (recomendado: 85%)
3. Configura el **intervalo de monitoreo** (recomendado: 0.2 segundos)
4. Haz clic en **"Iniciar"** para comenzar el monitoreo
5. **Minimizar a la bandeja** - la app funciona en segundo plano

## 🛠️ Características

- ✅ **Monitoreo Automático del Volumen**
- ✅ **Corrección de Volumen en Tiempo Real**
- ✅ **Integración en la Bandeja del Sistema**
- ✅ **Soporte Multiidioma**
- ✅ **Inicio con Windows**
- ✅ **Configuraciones Personalizables**
- ✅ **Bajo Uso de CPU**
- ✅ **Sin Servicios en Segundo Plano**

## 🎛️ Detalles Técnicos

### Cómo Funciona
1. **Monitorea** el dispositivo de micrófono seleccionado cada 0.2 segundos (configurable)
2. **Detecta** cuando el volumen cae por debajo del nivel objetivo
3. **Ajusta automáticamente** el volumen al nivel deseado
4. **Funciona silenciosamente** en la bandeja del sistema

### Requisitos del Sistema
- **Sistema Operativo**: Windows 10/11 (x64)
- **RAM**: 50MB
- **CPU**: Impacto mínimo (< 1%)
- **Permisos**: No requiere derechos de administrador

## 🔒 Privacidad y Seguridad

### Lo que esta App HACE:
- ✅ Monitorea los niveles de dispositivos de audio
- ✅ Ajusta el volumen del micrófono cuando es necesario
- ✅ Almacena configuraciones localmente en tu computadora
- ✅ Muestra notificaciones del sistema

### Lo que esta App NO hace:
- ❌ No graba ni escucha audio
- ❌ No accede a datos/streams del micrófono
- ❌ No envía datos a servidores externos
- ❌ No instala drivers o servicios del sistema
- ❌ No requiere permisos de administrador

### Garantía de Código Abierto
- **100% Código Abierto** - [ver todo el código](https://github.com/yourusername/razer-mic-fix)
- **Sin telemetría** o recopilación de datos
- **Sin funcionalidad oculta**
- **Verificado por la comunidad**

## 🚨 Aviso Importante

**Este software NO está afiliado con Razer Inc.**

- Esta es una **solución independiente creada por la comunidad**
- Desarrollada para ayudar a usuarios con problemas de micrófono
- **No se proporcionan garantías o aseguranzas**
- Úsalo bajo tu propio riesgo

## 🤝 Contribuir

¡Damos la bienvenida a las contribuciones! Así es como puedes ayudar:

### 🐛 Reportar Errores
- ¿Encontraste un error? [Crear un issue](https://github.com/yourusername/razer-mic-fix/issues)
- Incluye información del sistema y pasos para reproducir

### 💻 Contribuciones de Código
1. Hacer fork del repositorio
2. Crear rama de característica: `git checkout -b feature/caracteristica-increible`
3. Hacer commit de los cambios: `git commit -m 'Agregar característica increíble'`
4. Hacer push de la rama: `git push origin feature/caracteristica-increible`
5. Abrir Pull Request

### 🌐 Traducciones
Ayúdanos a traducir la app a más idiomas:
1. Copiar `src-tauri/src/i18n/en.json`
2. Traducir a tu idioma
3. Enviar Pull Request

## 🔨 Configuración de Desarrollo

### Prerrequisitos
- [Rust](https://rustup.rs/) (última versión estable)
- [Node.js](https://nodejs.org/) (16+)
- [Prerrequisitos de Tauri](https://tauri.app/v1/guides/getting-started/prerequisites)

### Compilar desde Código Fuente
```bash
# Clonar el repositorio
git clone https://github.com/yourusername/razer-mic-fix.git
cd razer-mic-fix

# Instalar dependencias
npm install

# Modo desarrollo
npm run tauri dev

# Compilar para producción
npm run tauri build
```

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](../LICENSE) para detalles.

---

**Hecho con ❤️ para la comunidad gaming**

*Última actualización: 09.2025*
