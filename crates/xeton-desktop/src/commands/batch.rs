use crate::commands::downloads::AppState;
use xeton_core::models::{DownloadProtocol, DuplicateStrategy, NewDownloadProps};
use xeton_models::{BatchDownloadRequest, DownloadItem};

#[tauri::command]
pub async fn add_batch_downloads(
    state: tauri::State<'_, AppState>,
    request: BatchDownloadRequest,
) -> Result<Vec<DownloadItem>, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let folder = request
        .output_dir
        .unwrap_or_else(|| format!("{}/Downloads", home));
    let mut added = Vec::new();

    for item in request.items {
        if !item.selected {
            continue;
        }

        let props = NewDownloadProps {
            folder: folder.clone(),
            name: item.file_name.clone(),
            link: item.url.clone(),
            protocol: DownloadProtocol::Http,
            preferred_connections: Some(8),
            speed_limit: 0,
            on_duplicate: DuplicateStrategy::AddNumbered,
        };

        if let Ok(id) = state.manager.add_download(props).await {
            if request.start_now {
                let _ = state.manager.resume(id).await;
            }

            let file_name = item.file_name.clone();
            added.push(DownloadItem {
                id: id.to_string(),
                name: file_name.clone(),
                url: item.url,
                category: item.category,
                status: xeton_models::DownloadStatus::Downloading,
                total_size: item.size,
                downloaded_size: 0,
                speed: 0,
                eta_seconds: None,
                connections: 8,
                date_added: chrono::Utc::now().timestamp(),
                date_finished: None,
                file_path: format!("{}/{}", folder.trim_end_matches('/'), file_name),
                error_message: None,
            });
        }
    }

    Ok(added)
}
