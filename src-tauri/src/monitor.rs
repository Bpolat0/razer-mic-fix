use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    time::Duration,
};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::audio;

#[derive(Clone)]
pub struct MonitorCfg {
    pub target: u32,
    pub interval_ms: u64,
    pub device_id: Option<String>,
    pub silent_log: bool,
}

pub struct MonitorHandle {
    stop: Arc<AtomicBool>,
    join: Option<std::thread::JoinHandle<()>>,
}

impl MonitorHandle {
    pub fn stop(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

#[derive(Default)]
pub struct MonitorState {
    pub handle: Option<MonitorHandle>,
    pub cfg: Option<MonitorCfg>,
}

pub static MONITOR: Lazy<Mutex<MonitorState>> = Lazy::new(|| Mutex::new(MonitorState::default()));

pub fn start(cfg: MonitorCfg) -> windows::core::Result<()> {
    println!("Monitor start called with cfg: target={}, interval={}, device_id={:?}", 
             cfg.target, cfg.interval_ms, cfg.device_id);
    stop();

    let cfg_for_state = cfg.clone(); // UI'de göstermek için saklayacağımız kopya
    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = stop.clone();

    let join = std::thread::spawn(move || {
        if !cfg.silent_log {
            println!("Monitor thread started");
        }
        unsafe {
            let _ = windows::Win32::System::Com::CoInitializeEx(
                None,
                windows::Win32::System::Com::COINIT_MULTITHREADED,
            );
        }

        let mut last_key = String::new();
        let mut vol: Option<windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume> = None;

        loop {
            if stop2.load(Ordering::Relaxed) { 
                println!("Monitor thread stopping");
                break; 
            }

            let key = cfg.device_id
                .as_deref()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .unwrap_or_else(default_id);

            if key != last_key || vol.is_none() {
                println!("Getting audio device for key: {}", key);
                vol = match cfg.device_id.as_deref().filter(|s| !s.is_empty())
                    .map(|_| audio::get_by_id(&key).map(|e| e.vol))
                    .unwrap_or_else(|| audio::get_default().map(|e| e.vol))
                {
                    Ok(v) => { 
                        println!("Successfully got audio device volume control");
                        last_key = key.clone(); 
                        Some(v) 
                    }
                    Err(e) => { 
                        println!("Failed to get audio device: {:?}", e);
                        vol.take(); 
                        std::thread::sleep(Duration::from_millis(200)); 
                        continue 
                    }
                };
            }

            if let Some(v) = &vol {
                if let Ok(cur) = audio::get_scalar(v) {
                    let t = cfg.target as f32 / 100.0;
                    if (cur - t).abs() > 0.005 {
                        println!("Adjusting volume from {:.2} to {:.2}", cur, t);
                        let _ = audio::set_scalar(v, t);
                    }
                } else {
                    println!("Failed to get current volume");
                    vol.take();
                }
            }

            std::thread::sleep(Duration::from_millis(cfg.interval_ms.max(20)));
        }

        unsafe { windows::Win32::System::Com::CoUninitialize(); }
        println!("Monitor thread finished");
    });

    let mut m = MONITOR.lock();
    m.cfg = Some(cfg_for_state);
    m.handle = Some(MonitorHandle { stop, join: Some(join) });
    println!("Monitor started successfully");
    Ok(())
}

pub fn stop() {
    println!("Monitor stop called");
    let mut m = MONITOR.lock();
    if let Some(h) = m.handle.take() {
        println!("Stopping monitor handle");
        h.stop();
        println!("Monitor stopped");
    } else {
        println!("No monitor handle to stop");
    }
}

pub fn running() -> bool { MONITOR.lock().handle.is_some() }

pub fn default_id() -> String {
    use windows::{
        core::Result as WinResult,
        Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator, EDataFlow, ERole},
        Win32::System::Com::{CoCreateInstance, CLSCTX_ALL},
    };
    unsafe {
        (|| -> WinResult<String> {
            let en: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
            let dev = en.GetDefaultAudioEndpoint(EDataFlow(1), ERole(2))?;
            let id_pw = dev.GetId()?;
            // PWSTR → String + free
            let mut len = 0;
            while *id_pw.0.add(len) != 0 { len += 1; }
            let s = String::from_utf16_lossy(std::slice::from_raw_parts(id_pw.0, len));
            windows::Win32::System::Com::CoTaskMemFree(Some(id_pw.0 as *const _));
            Ok(s)
        })().unwrap_or_default()
    }
}
