use std::sync::Arc;
use tokio::sync::RwLock;
use xeton_models::{AddDownloadRequest, DownloadItem, DownloadStatus, SpeedLimitConfig};

pub struct AppState {
    pub downloads: Arc<RwLock<Vec<DownloadItem>>>,
    pub speed_limit: Arc<RwLock<SpeedLimitConfig>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            downloads: Arc::new(RwLock::new(Vec::new())),
            speed_limit: Arc::new(RwLock::new(SpeedLimitConfig {
                enabled: false,
                max_bytes_per_sec: 0,
            })),
        }
    }
}

#[tauri::command]
pub async fn get_downloads(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<DownloadItem>, String> {
    let list = state.downloads.read().await;
    Ok(list.clone())
}

#[tauri::command]
pub async fn add_download(
    state: tauri::State<'_, AppState>,
    request: AddDownloadRequest,
) -> Result<DownloadItem, String> {
    let mut list = state.downloads.write().await;
    let item = DownloadItem {
        id: uuid::Uuid::new_v4().to_string(),
        name: request.file_name.unwrap_or_else(|| "download".into()),
        url: request.url,
        category: request.category.unwrap_or(xeton_models::CategoryType::Other),
        status: DownloadStatus::Downloading,
        total_size: None,
        downloaded_size: 0,
        speed: 0,
        eta_seconds: None,
        connections: request.connections.unwrap_or(8),
        date_added: chrono::Utc::now().timestamp(),
        date_finished: None,
        file_path: request.output_dir.unwrap_or_default(),
        error_message: None,
    };
    list.push(item.clone());
    Ok(item)
}

#[tauri::command]
pub async fn pause_download(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut list = state.downloads.write().await;
    if let Some(item) = list.iter_mut().find(|d| d.id == id) {
        item.status = DownloadStatus::Paused;
        item.speed = 0;
    }
    Ok(())
}

#[tauri::command]
pub async fn resume_download(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut list = state.downloads.write().await;
    if let Some(item) = list.iter_mut().find(|d| d.id == id) {
        item.status = DownloadStatus::Downloading;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_download(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut list = state.downloads.write().await;
    list.retain(|d| d.id != id);
    Ok(())
}
