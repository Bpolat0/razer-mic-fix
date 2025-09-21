use std::{ptr, slice, process::Command, collections::HashMap};

use windows::{
    core::{PCWSTR, PWSTR, Result as WinResult},
    Win32::{
        Media::Audio::{
            IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator,
            DEVICE_STATE_ACTIVE, EDataFlow, ERole,
        },
        Media::Audio::Endpoints::IAudioEndpointVolume,
        System::Com::{CoCreateInstance, CoTaskMemFree, CLSCTX_ALL},
    },
};

// Basit endpoint sarmalayıcı
pub struct Endpoint {
    #[allow(dead_code)]
    pub dev: IMMDevice,
    pub vol: IAudioEndpointVolume,
}

// PWSTR -> String + tahsisi serbest bırak
unsafe fn pwstr_to_string_and_free(pw: PWSTR) -> String {
    if pw.0.is_null() {
        return String::new();
    }
    let mut len = 0;
    while *pw.0.add(len) != 0 {
        len += 1;
    }
    let s = String::from_utf16_lossy(slice::from_raw_parts(pw.0, len));
    CoTaskMemFree(Some(pw.0 as *const std::ffi::c_void));
    s
}

// Windows Registry ve WMI kullanarak gerçek cihaz isimlerini al
fn get_real_device_names() -> HashMap<String, String> {
    let mut device_names = HashMap::new();
    
    // Çok basit PowerShell komutu kullananalım
    let ps_cmd = "Get-PnpDevice | Where { $_.Class -eq 'AudioEndpoint' } | % { $_.InstanceId + '|' + $_.FriendlyName }";

    println!("Getting device names via PnP...");
    println!("PowerShell command: {}", ps_cmd);
    
    if let Ok(output) = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy").arg("Bypass")
        .arg("-Command").arg(ps_cmd)
        .output()
    {
        println!("PowerShell exit status: {}", output.status);
        
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            println!("PowerShell stdout: '{}'", stdout);
            println!("PowerShell stdout length: {}", stdout.len());
            for (i, line) in stdout.lines().enumerate() {
                let line = line.trim();
                println!("Line {}: '{}'", i, line);
                if line.is_empty() {
                    continue;
                }
                
                if let Some((id, name)) = line.split_once('|') {
                    let clean_id = id.trim();
                    let clean_name = name.trim();
                    if !clean_name.is_empty() && clean_name != "Unknown" {
                        println!("Found device: {} -> {}", clean_id, clean_name);
                        device_names.insert(clean_id.to_string(), clean_name.to_string());
                        
                        // GUID eşleşmesi için de ekle - tam InstanceId'den GUID'i çıkar
                        if let Some(guid_start) = clean_id.rfind('{') {
                            let guid_part = &clean_id[guid_start..];
                            device_names.insert(guid_part.to_string(), clean_name.to_string());
                            println!("Added GUID mapping: {} -> {}", guid_part, clean_name);
                        }
                    }
                }
            }
        }
        
        if let Ok(stderr) = String::from_utf8(output.stderr) {
            if !stderr.trim().is_empty() {
                println!("PowerShell stderr: '{}'", stderr);
            }
        }
    }
    
    println!("Found {} device names total", device_names.len());
    device_names
}

// ID'den kullanıcı dostu isim türet
fn make_friendly_name(id: &str, device_map: &HashMap<String, String>) -> String {
    println!("Looking for friendly name for ID: {}", id);
    
    // Manuel mapping - bilinen cihazlar için
    if id.contains("6e694da6") || id.contains("6E694DA6") {
        return "Mikrofon (Razer BlackShark V2 Pro)".to_string();
    }
    if id.contains("a3f57a4e") || id.contains("A3F57A4E") {
        return "Mikrofon (Iriun Webcam)".to_string();
    }
    if id.contains("a6757a6d") || id.contains("A6757A6D") {
        return "AI Noise-Canceling Microphone (ASUS Utility)".to_string();
    }
    
    // Tam eşleşme
    if let Some(name) = device_map.get(id) {
        return name.clone();
    }
    
    // Partial eşleşme - GUID'leri karşılaştır
    if let Some(guid_start) = id.rfind('{') {
        let id_guid = &id[guid_start..];
        for (device_id, name) in device_map {
            if device_id.contains(id_guid) || id_guid.contains(device_id) {
                return name.clone();
            }
        }
    }
    
    // String içerik eşleşmesi
    for (device_id, name) in device_map {
        if id.contains(device_id) || device_id.contains(id) {
            return name.clone();
        }
    }
    
    // Hiç eşleşme yoksa basit isim
    if let Some(guid_start) = id.rfind('{') {
        let guid_part = &id[guid_start..];
        if guid_part.len() >= 9 {
            let short_guid = &guid_part[1..9];
            return format!("Audio Device {}", short_guid);
        }
    }
    
    "Audio Device".to_string()
}

// Default endpoint al
pub fn get_default() -> WinResult<Endpoint> {
    unsafe {
        let en: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let dev = en.GetDefaultAudioEndpoint(EDataFlow(1), ERole(2))?;
        let vol: IAudioEndpointVolume = dev.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)?;
        Ok(Endpoint { dev, vol })
    }
}

// ID ile cihaz
pub fn get_by_id(id: &str) -> WinResult<Endpoint> {
    unsafe {
        let en: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let wide: Vec<u16> = id.encode_utf16().chain(std::iter::once(0)).collect();
        let dev = en.GetDevice(PCWSTR(wide.as_ptr()))?;
        let vol: IAudioEndpointVolume = dev.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)?;
        Ok(Endpoint { dev, vol })
    }
}

// (isim, id) listesi — UI sadece ismi gösterir, değerde id durur
pub fn list_capture_names() -> WinResult<Vec<(String, String)>> {
    unsafe {
        // COM başlat
        let _ = windows::Win32::System::Com::CoInitialize(None);

        // Gerçek cihaz isimlerini al
        let device_names = get_real_device_names();

        let en: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let col = en.EnumAudioEndpoints(EDataFlow(1), DEVICE_STATE_ACTIVE)?;
        let count = col.GetCount()?;

        let mut out = Vec::with_capacity(count as usize);

        for i in 0..count {
            let dev = col.Item(i)?;
            let id_pw = dev.GetId()?;
            let id = pwstr_to_string_and_free(id_pw);
            
            println!("Device {}: ID = {}", i, id);
            
            // Gerçek ismi bul
            let friendly_name = make_friendly_name(&id, &device_names);
            
            println!("Device {}: Final name = {}", i, friendly_name);
            out.push((friendly_name, id));
        }
        Ok(out)
    }
}

pub fn get_scalar(vol: &IAudioEndpointVolume) -> WinResult<f32> {
    unsafe { Ok(vol.GetMasterVolumeLevelScalar()?) }
}

pub fn set_scalar(vol: &IAudioEndpointVolume, value: f32) -> WinResult<()> {
    unsafe { Ok(vol.SetMasterVolumeLevelScalar(value, ptr::null())?) }
}
