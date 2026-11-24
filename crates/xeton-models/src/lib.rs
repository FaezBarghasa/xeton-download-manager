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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PowerAction {
    None,
    Shutdown,
    Sleep,
    Hibernate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub name: String,
    pub max_concurrent: u32,
    pub is_active: bool,
    pub scheduled_start: Option<String>, // e.g. "02:00"
    pub scheduled_stop: Option<String>,  // e.g. "07:00"
    pub power_action: PowerAction,
    pub items_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomCategory {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub extensions: Vec<String>,
    pub default_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChecksumAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumRequest {
    pub file_path: String,
    pub algorithm: ChecksumAlgorithm,
    pub expected_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumResult {
    pub file_path: String,
    pub hash: String,
    pub algorithm: ChecksumAlgorithm,
    pub matches: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerHostConfig {
    pub host: String,
    pub max_connections: u32,
    pub speed_limit_bytes_per_sec: Option<u64>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartProgress {
    pub part_id: u32,
    pub start_byte: u64,
    pub current_byte: u64,
    pub end_byte: u64,
    pub speed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadPartsInfo {
    pub download_id: String,
    pub total_size: u64,
    pub parts: Vec<PartProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDownloadItem {
    pub url: String,
    pub file_name: String,
    pub size: Option<u64>,
    pub selected: bool,
    pub category: CategoryType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDownloadRequest {
    pub items: Vec<BatchDownloadItem>,
    pub output_dir: Option<String>,
    pub queue_id: Option<String>,
    pub start_now: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullAppSettings {
    pub theme: String,
    pub ui_scale: f32,
    pub language: String,
    pub max_concurrent_downloads: u32,
    pub default_connections_per_download: u32,
    pub speed_limit_enabled: bool,
    pub max_download_speed: u64,
    pub default_download_dir: String,
    pub duplicate_strategy: String,
    pub auto_start_on_boot: bool,
    pub minimize_to_tray: bool,
    pub prevent_sleep: bool,
    pub shutdown_on_completion: bool,
    pub browser_integration_port: u16,
    pub intercept_clipboard: bool,
}

impl Default for FullAppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".into(),
            ui_scale: 1.0,
            language: "en".into(),
            max_concurrent_downloads: 3,
            default_connections_per_download: 8,
            speed_limit_enabled: false,
            max_download_speed: 0,
            default_download_dir: "~/Downloads".into(),
            duplicate_strategy: "rename".into(),
            auto_start_on_boot: false,
            minimize_to_tray: true,
            prevent_sleep: true,
            shutdown_on_completion: false,
            browser_integration_port: 8991,
            intercept_clipboard: true,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_item_serialization() {
        let item = DownloadItem {
            id: "dl-101".into(),
            name: "test-file.zip".into(),
            url: "https://example.com/test-file.zip".into(),
            category: CategoryType::Compressed,
            status: DownloadStatus::Downloading,
            total_size: Some(10485760),
            downloaded_size: 5242880,
            speed: 1048576,
            eta_seconds: Some(5),
            connections: 8,
            date_added: 1732431000,
            date_finished: None,
            file_path: "/tmp/test-file.zip".into(),
            error_message: None,
        };

        let json = serde_json::to_string(&item).expect("Failed to serialize");
        let deserialized: DownloadItem = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(item, deserialized);
        assert_eq!(deserialized.category, CategoryType::Compressed);
        assert_eq!(deserialized.status, DownloadStatus::Downloading);
    }
}
