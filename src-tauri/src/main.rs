#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod audio;
mod monitor;
mod settings;
mod i18n;

use serde::Serialize;
use tauri::{Manager, tray::{TrayIconBuilder, TrayIconEvent}};
use tauri_plugin_notification::NotificationExt;
use settings::AppSettings;
use i18n::{I18n, Translations};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use single_instance::SingleInstance;

static I18N: Lazy<Mutex<I18n>> = Lazy::new(|| Mutex::new(I18n::new()));

#[derive(Serialize, Debug)]
struct DeviceInfo { name: String, id: String }

#[derive(Serialize, Debug)]
struct LanguageInfo { code: String, name: String }

#[tauri::command]
fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    #[cfg(debug_assertions)]
    println!("list_devices command called");
    let result = audio::list_capture_names()
        .map(|v| v.into_iter().map(|(n,id)| DeviceInfo { name: n, id }).collect())
        .map_err(|e| format!("{e:?}"));
    #[cfg(debug_assertions)]
    println!("list_devices result: {:?}", result);
    result
}

#[tauri::command]
fn start_monitor(target: u32, interval_ms: u64, device_id: Option<String>) -> Result<(), String> {
    monitor::start(monitor::MonitorCfg { target, interval_ms, device_id, silent_log: false }).map_err(|e| format!("{e:?}"))
}

#[tauri::command] 
fn stop_monitor() { 
    monitor::stop(); 
}

#[tauri::command] 
fn is_running() -> bool { 
    monitor::running() 
}

#[tauri::command] 
fn get_default_device_id() -> String { 
    monitor::default_id() 
}

#[tauri::command]
fn save_settings(settings: AppSettings) -> Result<(), String> {
    settings.save().map_err(|e| format!("{e:?}"))
}

#[tauri::command]
fn load_settings() -> AppSettings {
    AppSettings::load()
}

#[tauri::command]
fn get_translations(language: String) -> Option<Translations> {
    let mut i18n = I18N.lock().unwrap();
    i18n.set_language(&language);
    i18n.get_translations().cloned()
}

#[tauri::command]
fn get_available_languages() -> Vec<LanguageInfo> {
    let i18n = I18N.lock().unwrap();
    i18n.get_available_languages()
        .into_iter()
        .map(|(code, name)| LanguageInfo { code, name })
        .collect()
}

#[tauri::command]
fn show_notification(title: String, body: String, app: tauri::AppHandle) -> Result<(), String> {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .icon("icons/mic.ico")
        .show()
        .map_err(|e| format!("{e:?}"))
}

#[tauri::command]
fn toggle_autostart(enable: bool, app: tauri::AppHandle) -> Result<bool, String> {
    match app.try_state::<tauri_plugin_autostart::AutoLaunchManager>() {
        Some(manager) => {
            if enable {
                manager.enable().map_err(|e| format!("{e:?}"))?;
            } else {
                manager.disable().map_err(|e| format!("{e:?}"))?;
            }
            Ok(enable)
        }
        None => Err("Autostart manager not available".to_string())
    }
}

#[tauri::command]
fn is_autostart_enabled(app: tauri::AppHandle) -> Result<bool, String> {
    match app.try_state::<tauri_plugin_autostart::AutoLaunchManager>() {
        Some(manager) => {
            manager.is_enabled().map_err(|e| format!("{e:?}"))
        },
        None => {
            Err("Autostart manager not available".to_string())
        }
    }
}

#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    println!("Quitting application via quit_app command...");
    app.exit(0);
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) -> Result<(), String> {
    println!("Hiding window to tray...");
    match app.get_webview_window("main") {
        Some(window) => {
            window.hide().map_err(|e| format!("Failed to hide window: {e:?}"))?;
            Ok(())
        }
        None => Err("Main window not found".to_string())
    }
}

#[tauri::command]
fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    println!("Showing main window...");
    match app.get_webview_window("main") {
        Some(window) => {
            window.show().map_err(|e| format!("Failed to show window: {e:?}"))?;
            window.set_focus().map_err(|e| format!("Failed to focus window: {e:?}"))?;
            Ok(())
        }
        None => Err("Main window not found".to_string())
    }
}

fn main() {
    // Single instance kontrolü
    let instance = SingleInstance::new("razer-mic-fix-app").unwrap();
    if !instance.is_single() {
        eprintln!("Another instance is already running!");
        std::process::exit(1);
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent, 
            Some(vec!["--silent"])
        ))
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_handle_clone = app_handle.clone();
            let app_handle_quit = app_handle.clone();
            
            println!("Creating tray icon...");
            let tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Razer Mic Volume Fixer")
                .on_tray_icon_event(move |_tray, event| {
                    #[cfg(debug_assertions)]
                    println!("Tray icon event: {:?}", event);
                    if let TrayIconEvent::DoubleClick { .. } = event {
                        let _ = app_handle_clone.get_webview_window("main").unwrap().show();
                        let _ = app_handle_clone.get_webview_window("main").unwrap().set_focus();
                    }
                })
                .menu(&tauri::menu::Menu::with_items(
                    &app_handle,
                    &[
                        &tauri::menu::MenuItem::with_id(&app_handle, "show", "Show", true, None::<&str>).unwrap(),
                        &tauri::menu::MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap()
                    ]
                ).unwrap())
                .on_menu_event(move |_tray, e| {
                    #[cfg(debug_assertions)]
                    println!("Tray menu event: {:?}", e.id());
                    match e.id().as_ref() {
                        "show" => { 
                            let _ = app_handle.get_webview_window("main").unwrap().show(); 
                            let _ = app_handle.get_webview_window("main").unwrap().set_focus();
                        }
                        "quit" => { 
                            #[cfg(debug_assertions)]
                            println!("Quitting application...");
                            app_handle_quit.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            // Tray icon'u app state'e sakla
            app.manage(tray);
            println!("Tray icon created successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_devices, start_monitor, stop_monitor, is_running, get_default_device_id,
            save_settings, load_settings, get_translations, get_available_languages, show_notification,
            toggle_autostart, is_autostart_enabled, quit_app, hide_window, show_main_window
        ])
        .on_window_event(|w, e| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                api.prevent_close(); 
                let _ = w.hide(); // X'e basınca tepsiye
                
                // Notification göster
                let app = w.app_handle();
                let _ = app.notification()
                    .builder()
                    .title("Razer Mic Volume Fixer")
                    .body("Uygulama sistem tepsisine gizlendi")
                    .icon("icons/mic.ico")
                    .show();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
