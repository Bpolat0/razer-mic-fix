use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub vol: u32,
    pub intv: f64,
    pub dev: String,
    pub language: String,
    pub autostart: bool,
    #[serde(rename = "startMonitoring")]
    pub start_monitoring: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            vol: 85,
            intv: 0.2,
            dev: String::new(),
            language: "en".to_string(),
            autostart: false,
            start_monitoring: false,
        }
    }
}

impl AppSettings {
    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("RazerMicVolumeFixer");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        Ok(config_dir.join("settings.json"))
    }
    
    pub fn load() -> Self {
        match Self::get_config_path() {
            Ok(path) => {
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            match serde_json::from_str::<AppSettings>(&content) {
                                Ok(settings) => {
                                    println!("Loaded settings from: {}", path.display());
                                    return settings;
                                }
                                Err(e) => println!("Failed to parse settings: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read settings file: {}", e),
                    }
                }
            }
            Err(e) => println!("Failed to get config path: {}", e),
        }
        
        println!("Using default settings");
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        println!("Settings saved successfully");
        Ok(())
    }
}
