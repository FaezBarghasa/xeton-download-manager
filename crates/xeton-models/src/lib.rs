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
