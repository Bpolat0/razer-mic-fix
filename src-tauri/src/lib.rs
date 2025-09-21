#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod monitor;

use serde::Serialize;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[derive(Serialize)]
struct DeviceInfo {
    name: String,
    id: String,
}

#[tauri::command]
fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    audio::list_capture_names()
        .map(|v| v.into_iter().map(|(name, id)| DeviceInfo { name, id }).collect())
        .map_err(|e| format!("{e:?}"))
}

#[tauri::command]
fn start_monitor(target: u32, interval_ms: u64, device_id: Option<String>) -> Result<(), String> {
    monitor::start(monitor::MonitorCfg { target, interval_ms, device_id, silent_log: false })
        .map_err(|e| format!("{e:?}"))
}

#[tauri::command]
fn stop_monitor() { monitor::stop(); }

#[tauri::command]
fn is_running() -> bool { monitor::running() }

#[tauri::command]
fn get_default_device_id() -> String { monitor::default_id() }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent, None
        ))
        .setup(|app| {
            // Tray kurulumunu burada yapalım; Box<dyn Error> dönmeli
            init_tray(&app.handle()).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })
        })
        .invoke_handler(tauri::generate_handler![
            list_devices,
            start_monitor,
            stop_monitor,
            is_running,
            get_default_device_id
        ])
        .on_window_event(|w, e| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                api.prevent_close(); // X → tray
                let _ = w.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let tray_menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

    TrayIconBuilder::new()
        .menu(&tray_menu)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                if let Some(win) = tray.app_handle().get_webview_window("main") {
                    let _ = win.show();
                    let _ = win.set_focus();
                }
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.show();
                    let _ = win.set_focus();
                }
            }
            "quit" => std::process::exit(0),
            _ => {}
        })
        .build(app)?;
    Ok(())
}
