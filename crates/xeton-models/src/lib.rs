use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
    Scheduled,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CategoryType {
    All,
    Compressed,
    Documents,
    Music,
    Programs,
    Videos,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: String,
    pub name: String,
    pub url: String,
    pub category: CategoryType,
    pub status: DownloadStatus,
    pub total_size: Option<u64>,
    pub downloaded_size: u64,
    pub speed: u64, // bytes per second
    pub eta_seconds: Option<u64>,
    pub connections: u32,
    pub date_added: i64,
    pub date_finished: Option<i64>,
    pub file_path: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgressTick {
    pub id: String,
    pub downloaded_size: u64,
    pub speed: u64,
    pub eta_seconds: Option<u64>,
    pub status: DownloadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDownloadRequest {
    pub url: String,
    pub file_name: Option<String>,
    pub output_dir: Option<String>,
    pub category: Option<CategoryType>,
    pub connections: Option<u32>,
    pub start_now: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppThemeConfig {
    pub theme_id: String, // "dark", "light", "obsidian"
    pub ui_scale: f32,
    pub font_family: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedLimitConfig {
    pub enabled: bool,
    pub max_bytes_per_sec: u64,
}
