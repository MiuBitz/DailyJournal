use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub const APP_NAME: &str = "Daily Journal";

fn default_morning_time() -> String {
    "12:00".to_string()
}
fn default_afternoon_time() -> String {
    "18:00".to_string()
}
fn default_evening_time() -> String {
    "22:00".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemindersConfig {
    pub morning: bool,
    pub afternoon: bool,
    pub evening: bool,

    #[serde(default = "default_morning_time")]
    pub morning_time: String,

    #[serde(default = "default_afternoon_time")]
    pub afternoon_time: String,

    #[serde(default = "default_evening_time")]
    pub evening_time: String,
}

impl Default for RemindersConfig {
    fn default() -> Self {
        Self {
            morning: true,
            afternoon: true,
            evening: true,
            morning_time: default_morning_time(),
            afternoon_time: default_afternoon_time(),
            evening_time: default_evening_time(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub output_folder: String,
    pub reminders: RemindersConfig,
    pub start_with_windows: bool,
    pub minimize_to_tray: bool,
    pub last_reminded: HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let default_output = dirs_next_output_folder();
        Self {
            output_folder: default_output,
            reminders: RemindersConfig::default(),
            start_with_windows: true,
            minimize_to_tray: true,
            last_reminded: HashMap::new(),
        }
    }
}

pub fn config_dir() -> PathBuf {
    if let Some(app_data) = std::env::var_os("APPDATA") {
        PathBuf::from(app_data).join("DailyJournal")
    } else if let Some(home) = dirs_home_dir() {
        home.join("DailyJournal")
    } else {
        PathBuf::from("DailyJournal")
    }
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.json")
}

pub fn dirs_home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

pub fn dirs_next_output_folder() -> String {
    if let Some(home) = dirs_home_dir() {
        home.join("Documents")
            .join("Daily Journal")
            .to_string_lossy()
            .to_string()
    } else {
        String::from("Daily Journal")
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let path = config_file();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str::<AppConfig>(&content) {
                    return cfg;
                }
            }
        }
        let default_cfg = AppConfig::default();
        default_cfg.save();
        default_cfg
    }

    pub fn save(&self) {
        let dir = config_dir();
        let _ = fs::create_dir_all(&dir);
        let path = config_file();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}
