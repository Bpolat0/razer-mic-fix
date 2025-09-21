use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub app_title: String,
    pub target_volume: String,
    pub check_interval: String,
    pub device: String,
    pub start_monitoring_on_launch: String,
    pub start_with_windows: String,
    pub silent_log: String,
    pub language: String,
    pub start: String,
    pub stop: String,
    pub hide_to_tray: String,
    pub exit: String,
    pub default_windows: String,
    pub notification_hidden: String,
    pub notification_monitoring_started: String,
    pub notification_monitoring_stopped: String,
}

pub struct I18n {
    translations: HashMap<String, Translations>,
    current_language: String,
}

impl I18n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // English
        translations.insert("en".to_string(), Translations {
            app_title: "Razer Mic Volume Fixer".to_string(),
            target_volume: "Target Volume (%)".to_string(),
            check_interval: "Check interval (s)".to_string(),
            device: "Device".to_string(),
            start_monitoring_on_launch: "Start monitoring on launch".to_string(),
            start_with_windows: "Start with Windows".to_string(),
            silent_log: "Silent log (reduces console output)".to_string(),
            language: "Language".to_string(),
            start: "Start".to_string(),
            stop: "Stop".to_string(),
            hide_to_tray: "Hide to Tray".to_string(),
            exit: "Exit".to_string(),
            default_windows: "Default (Windows)".to_string(),
            notification_hidden: "Application hidden to system tray".to_string(),
            notification_monitoring_started: "Volume monitoring started".to_string(),
            notification_monitoring_stopped: "Volume monitoring stopped".to_string(),
        });
        
        // Turkish
        translations.insert("tr".to_string(), Translations {
            app_title: "Razer Mikrofon Ses Düzeltici".to_string(),
            target_volume: "Hedef Ses (%%)".to_string(),
            check_interval: "Kontrol aralığı (s)".to_string(),
            device: "Cihaz".to_string(),
            start_monitoring_on_launch: "Açılışta izlemeyi başlat".to_string(),
            start_with_windows: "Windows ile başlat".to_string(),
            silent_log: "Sessiz log (konsol çıktısını azaltır)".to_string(),
            language: "Dil".to_string(),
            start: "Başlat".to_string(),
            stop: "Durdur".to_string(),
            hide_to_tray: "Sistem Tepsisine Gizle".to_string(),
            exit: "Çıkış".to_string(),
            default_windows: "Varsayılan (Windows)".to_string(),
            notification_hidden: "Uygulama sistem tepsisine gizlendi".to_string(),
            notification_monitoring_started: "Ses izleme başlatıldı".to_string(),
            notification_monitoring_stopped: "Ses izleme durduruldu".to_string(),
        });
        
        // German
        translations.insert("de".to_string(), Translations {
            app_title: "Razer Mikrofon Lautstärke Fixierer".to_string(),
            target_volume: "Ziel-Lautstärke (%)".to_string(),
            check_interval: "Prüfintervall (s)".to_string(),
            device: "Gerät".to_string(),
            start_monitoring_on_launch: "Überwachung beim Start".to_string(),
            start_with_windows: "Mit Windows starten".to_string(),
            silent_log: "Stilles Protokoll".to_string(),
            language: "Sprache".to_string(),
            start: "Starten".to_string(),
            stop: "Stoppen".to_string(),
            hide_to_tray: "In Taskleiste ausblenden".to_string(),
            exit: "Beenden".to_string(),
            default_windows: "Standard (Windows)".to_string(),
            notification_hidden: "Anwendung in Taskleiste ausgeblendet".to_string(),
            notification_monitoring_started: "Lautstärkeüberwachung gestartet".to_string(),
            notification_monitoring_stopped: "Lautstärkeüberwachung gestoppt".to_string(),
        });
        
        // French
        translations.insert("fr".to_string(), Translations {
            app_title: "Correcteur de Volume Micro Razer".to_string(),
            target_volume: "Volume Cible (%)".to_string(),
            check_interval: "Intervalle de vérification (s)".to_string(),
            device: "Appareil".to_string(),
            start_monitoring_on_launch: "Démarrer la surveillance au lancement".to_string(),
            start_with_windows: "Démarrer avec Windows".to_string(),
            silent_log: "Journal silencieux".to_string(),
            language: "Langue".to_string(),
            start: "Démarrer".to_string(),
            stop: "Arrêter".to_string(),
            hide_to_tray: "Masquer dans la barre".to_string(),
            exit: "Quitter".to_string(),
            default_windows: "Défaut (Windows)".to_string(),
            notification_hidden: "Application masquée dans la barre système".to_string(),
            notification_monitoring_started: "Surveillance du volume démarrée".to_string(),
            notification_monitoring_stopped: "Surveillance du volume arrêtée".to_string(),
        });
        
        // Spanish
        translations.insert("es".to_string(), Translations {
            app_title: "Reparador de Volumen de Micrófono Razer".to_string(),
            target_volume: "Volumen Objetivo (%)".to_string(),
            check_interval: "Intervalo de verificación (s)".to_string(),
            device: "Dispositivo".to_string(),
            start_monitoring_on_launch: "Iniciar monitoreo al arrancar".to_string(),
            start_with_windows: "Iniciar con Windows".to_string(),
            silent_log: "Registro silencioso".to_string(),
            language: "Idioma".to_string(),
            start: "Iniciar".to_string(),
            stop: "Detener".to_string(),
            hide_to_tray: "Ocultar en bandeja".to_string(),
            exit: "Salir".to_string(),
            default_windows: "Predeterminado (Windows)".to_string(),
            notification_hidden: "Aplicación ocultada en la bandeja del sistema".to_string(),
            notification_monitoring_started: "Monitoreo de volumen iniciado".to_string(),
            notification_monitoring_stopped: "Monitoreo de volumen detenido".to_string(),
        });
        
        // Italian
        translations.insert("it".to_string(), Translations {
            app_title: "Correttore Volume Microfono Razer".to_string(),
            target_volume: "Volume Target (%)".to_string(),
            check_interval: "Intervallo di controllo (s)".to_string(),
            device: "Dispositivo".to_string(),
            start_monitoring_on_launch: "Avvia monitoraggio all'avvio".to_string(),
            start_with_windows: "Avvia con Windows".to_string(),
            silent_log: "Log silenzioso".to_string(),
            language: "Lingua".to_string(),
            start: "Avvia".to_string(),
            stop: "Ferma".to_string(),
            hide_to_tray: "Nascondi nella barra".to_string(),
            exit: "Esci".to_string(),
            default_windows: "Predefinito (Windows)".to_string(),
            notification_hidden: "Applicazione nascosta nella barra di sistema".to_string(),
            notification_monitoring_started: "Monitoraggio volume avviato".to_string(),
            notification_monitoring_stopped: "Monitoraggio volume fermato".to_string(),
        });
        
        // Russian
        translations.insert("ru".to_string(), Translations {
            app_title: "Исправитель громкости микрофона Razer".to_string(),
            target_volume: "Целевая громкость (%)".to_string(),
            check_interval: "Интервал проверки (с)".to_string(),
            device: "Устройство".to_string(),
            start_monitoring_on_launch: "Запускать мониторинг при старте".to_string(),
            start_with_windows: "Запускать с Windows".to_string(),
            silent_log: "Тихий журнал".to_string(),
            language: "Язык".to_string(),
            start: "Запустить".to_string(),
            stop: "Остановить".to_string(),
            hide_to_tray: "Скрыть в трей".to_string(),
            exit: "Выход".to_string(),
            default_windows: "По умолчанию (Windows)".to_string(),
            notification_hidden: "Приложение скрыто в системный трей".to_string(),
            notification_monitoring_started: "Мониторинг громкости запущен".to_string(),
            notification_monitoring_stopped: "Мониторинг громкости остановлен".to_string(),
        });
        
        // Japanese
        translations.insert("ja".to_string(), Translations {
            app_title: "Razer マイクボリューム修正ツール".to_string(),
            target_volume: "目標音量 (%)".to_string(),
            check_interval: "チェック間隔 (秒)".to_string(),
            device: "デバイス".to_string(),
            start_monitoring_on_launch: "起動時にモニタリング開始".to_string(),
            start_with_windows: "Windowsと一緒に開始".to_string(),
            silent_log: "サイレントログ".to_string(),
            language: "言語".to_string(),
            start: "開始".to_string(),
            stop: "停止".to_string(),
            hide_to_tray: "トレイに隠す".to_string(),
            exit: "終了".to_string(),
            default_windows: "デフォルト (Windows)".to_string(),
            notification_hidden: "アプリケーションがシステムトレイに隠されました".to_string(),
            notification_monitoring_started: "音量モニタリングが開始されました".to_string(),
            notification_monitoring_stopped: "音量モニタリングが停止されました".to_string(),
        });
        
        // Chinese Simplified
        translations.insert("zh".to_string(), Translations {
            app_title: "雷蛇麦克风音量修复器".to_string(),
            target_volume: "目标音量 (%)".to_string(),
            check_interval: "检查间隔 (秒)".to_string(),
            device: "设备".to_string(),
            start_monitoring_on_launch: "启动时开始监控".to_string(),
            start_with_windows: "随Windows启动".to_string(),
            silent_log: "静默日志".to_string(),
            language: "语言".to_string(),
            start: "开始".to_string(),
            stop: "停止".to_string(),
            hide_to_tray: "隐藏到托盘".to_string(),
            exit: "退出".to_string(),
            default_windows: "默认 (Windows)".to_string(),
            notification_hidden: "应用程序已隐藏到系统托盘".to_string(),
            notification_monitoring_started: "音量监控已开始".to_string(),
            notification_monitoring_stopped: "音量监控已停止".to_string(),
        });
        
        // Korean
        translations.insert("ko".to_string(), Translations {
            app_title: "Razer 마이크 볼륨 수정기".to_string(),
            target_volume: "목표 볼륨 (%)".to_string(),
            check_interval: "확인 간격 (초)".to_string(),
            device: "장치".to_string(),
            start_monitoring_on_launch: "시작 시 모니터링 시작".to_string(),
            start_with_windows: "Windows와 함께 시작".to_string(),
            silent_log: "무음 로그".to_string(),
            language: "언어".to_string(),
            start: "시작".to_string(),
            stop: "중지".to_string(),
            hide_to_tray: "트레이로 숨기기".to_string(),
            exit: "종료".to_string(),
            default_windows: "기본값 (Windows)".to_string(),
            notification_hidden: "애플리케이션이 시스템 트레이로 숨겨졌습니다".to_string(),
            notification_monitoring_started: "볼륨 모니터링이 시작되었습니다".to_string(),
            notification_monitoring_stopped: "볼륨 모니터링이 중지되었습니다".to_string(),
        });
        
        Self {
            translations,
            current_language: "en".to_string(),
        }
    }
    
    pub fn set_language(&mut self, lang: &str) {
        if self.translations.contains_key(lang) {
            self.current_language = lang.to_string();
        }
    }
    
    pub fn get_translations(&self) -> Option<&Translations> {
        self.translations.get(&self.current_language)
    }
    
    pub fn get_available_languages(&self) -> Vec<(String, String)> {
        vec![
            ("en".to_string(), "English".to_string()),
            ("tr".to_string(), "Türkçe".to_string()),
            ("de".to_string(), "Deutsch".to_string()),
            ("fr".to_string(), "Français".to_string()),
            ("es".to_string(), "Español".to_string()),
            ("it".to_string(), "Italiano".to_string()),
            ("ru".to_string(), "Русский".to_string()),
            ("ja".to_string(), "日本語".to_string()),
            ("zh".to_string(), "中文".to_string()),
            ("ko".to_string(), "한국어".to_string()),
        ]
    }
}
