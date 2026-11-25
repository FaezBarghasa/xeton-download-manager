use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use xeton_core::manager::DownloadManager;
use xeton_core::models::{DownloadProtocol, DownloadSettings, DuplicateStrategy, NewDownloadProps};
use xeton_models::{
    AddDownloadRequest, CategoryType, DownloadItem as UiDownloadItem,
    DownloadStatus as UiDownloadStatus, SpeedLimitConfig,
};

pub struct AppState {
    pub manager: Arc<DownloadManager>,
    #[allow(dead_code)]
    pub speed_limit: Arc<RwLock<SpeedLimitConfig>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        eprintln!("[XETON] AppState::new: creating data_dir");
        #[cfg(target_os = "android")]
        let data_dir = {
            let files_dir = PathBuf::from("/data/data/com.xeton.downloadmanager/files");
            let _ = tokio::fs::create_dir_all(&files_dir).await;
            files_dir.join("data")
        };
        #[cfg(not(target_os = "android"))]
        let data_dir = {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".local/share/xeton-download-manager/data")
        };
        tokio::fs::create_dir_all(&data_dir).await?;

        eprintln!(
            "[XETON] AppState::new: initializing DownloadManager at {:?}",
            data_dir
        );
        let settings = DownloadSettings::default();
        let manager = DownloadManager::new(data_dir, settings).await?;
        eprintln!("[XETON] AppState::new: booting DownloadManager");
        manager.boot().await?;
        eprintln!("[XETON] AppState::new: DownloadManager booted");

        Ok(Self {
            manager,
            speed_limit: Arc::new(RwLock::new(SpeedLimitConfig {
                enabled: false,
                max_bytes_per_sec: 0,
            })),
        })
    }
}

#[tauri::command]
pub async fn get_downloads(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<UiDownloadItem>, String> {
    let list = state
        .manager
        .get_download_list()
        .await
        .map_err(|e| e.to_string())?;

    let mut ui_items = Vec::with_capacity(list.len());
    for item in list {
        let status = match item.status {
            xeton_core::models::DownloadStatus::Downloading => UiDownloadStatus::Downloading,
            xeton_core::models::DownloadStatus::Completed => UiDownloadStatus::Completed,
            xeton_core::models::DownloadStatus::Paused => UiDownloadStatus::Paused,
            xeton_core::models::DownloadStatus::Error => UiDownloadStatus::Failed,
            _ => UiDownloadStatus::Pending,
        };

        let category = if item.name.ends_with(".zip")
            || item.name.ends_with(".tar.gz")
            || item.name.ends_with(".iso")
        {
            CategoryType::Compressed
        } else if item.name.ends_with(".mp4") || item.name.ends_with(".mkv") {
            CategoryType::Videos
        } else if item.name.ends_with(".mp3") || item.name.ends_with(".flac") {
            CategoryType::Music
        } else if item.name.ends_with(".pdf") || item.name.ends_with(".doc") {
            CategoryType::Documents
        } else if item.name.ends_with(".deb")
            || item.name.ends_with(".exe")
            || item.name.ends_with(".AppImage")
        {
            CategoryType::Programs
        } else {
            CategoryType::Other
        };

        let parts = state
            .manager
            .get_download_parts(item.numeric_id)
            .await
            .unwrap_or_default();
        let downloaded_size: u64 = parts.iter().map(|p| p.downloaded().max(0) as u64).sum();

        let file_path = format!("{}/{}", item.folder.trim_end_matches('/'), item.name);

        ui_items.push(UiDownloadItem {
            id: item.numeric_id.to_string(),
            name: item.name,
            url: item.link,
            category,
            status,
            total_size: if item.content_length > 0 {
                Some(item.content_length as u64)
            } else {
                None
            },
            downloaded_size,
            speed: 0,
            eta_seconds: None,
            connections: item.preferred_connections.unwrap_or(8),
            date_added: item.date_added,
            date_finished: item.complete_time,
            file_path,
            error_message: None,
        });
    }

    Ok(ui_items)
}

#[tauri::command]
pub async fn add_download(
    state: tauri::State<'_, AppState>,
    request: AddDownloadRequest,
) -> Result<UiDownloadItem, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let folder = request
        .output_dir
        .unwrap_or_else(|| format!("{}/Downloads", home));
    let name = request.file_name.unwrap_or_else(|| {
        request
            .url
            .split('/')
            .last()
            .unwrap_or("download")
            .to_string()
    });

    let props = NewDownloadProps {
        folder: folder.clone(),
        name: name.clone(),
        link: request.url.clone(),
        protocol: DownloadProtocol::Http,
        preferred_connections: request.connections,
        speed_limit: 0,
        on_duplicate: DuplicateStrategy::AddNumbered,
    };

    let id = state
        .manager
        .add_download(props)
        .await
        .map_err(|e| e.to_string())?;

    if request.start_now {
        let _ = state.manager.resume(id).await;
    }

    Ok(UiDownloadItem {
        id: id.to_string(),
        name: name.clone(),
        url: request.url,
        category: request.category.unwrap_or(CategoryType::Other),
        status: UiDownloadStatus::Downloading,
        total_size: None,
        downloaded_size: 0,
        speed: 0,
        eta_seconds: None,
        connections: request.connections.unwrap_or(8),
        date_added: chrono::Utc::now().timestamp(),
        date_finished: None,
        file_path: format!("{}/{}", folder.trim_end_matches('/'), name),
        error_message: None,
    })
}

#[tauri::command]
pub async fn pause_download(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    if let Ok(numeric_id) = id.parse::<i64>() {
        state
            .manager
            .pause(numeric_id)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn resume_download(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    if let Ok(numeric_id) = id.parse::<i64>() {
        state
            .manager
            .resume(numeric_id)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_download(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    if let Ok(numeric_id) = id.parse::<i64>() {
        state
            .manager
            .delete_download(numeric_id, false)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
