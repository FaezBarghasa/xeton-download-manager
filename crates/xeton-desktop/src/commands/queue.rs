use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use xeton_models::{PowerAction, QueueItem};

fn queues_path() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".local").join("share")
        });
    base.join("xeton-download-manager").join("queues.json")
}

async fn persist_queues(queues: &[QueueItem]) -> Result<(), String> {
    let path = queues_path();
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let json = serde_json::to_string_pretty(queues).map_err(|e| e.to_string())?;
    tokio::fs::write(&path, json)
        .await
        .map_err(|e| e.to_string())
}

#[derive(Default)]
pub struct QueueState {
    pub queues: Arc<RwLock<Vec<QueueItem>>>,
}

impl QueueState {
    pub fn new() -> Self {
        let path = queues_path();
        let initial = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str::<Vec<QueueItem>>(&s).ok())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| {
                vec![
                    QueueItem {
                        id: "queue-default".into(),
                        name: "Default Queue".into(),
                        max_concurrent: 3,
                        is_active: true,
                        scheduled_start: None,
                        scheduled_stop: None,
                        power_action: PowerAction::None,
                        items_count: 0,
                    },
                    QueueItem {
                        id: "queue-night".into(),
                        name: "Night Schedule (02:00 - 07:00)".into(),
                        max_concurrent: 5,
                        is_active: false,
                        scheduled_start: Some("02:00".into()),
                        scheduled_stop: Some("07:00".into()),
                        power_action: PowerAction::Sleep,
                        items_count: 0,
                    },
                ]
            });
        Self {
            queues: Arc::new(RwLock::new(initial)),
        }
    }
}

#[tauri::command]
pub async fn get_queues(state: tauri::State<'_, QueueState>) -> Result<Vec<QueueItem>, String> {
    Ok(state.queues.read().await.clone())
}

#[tauri::command]
pub async fn create_queue(
    state: tauri::State<'_, QueueState>,
    name: String,
    max_concurrent: u32,
    scheduled_start: Option<String>,
    scheduled_stop: Option<String>,
    power_action: PowerAction,
) -> Result<QueueItem, String> {
    let id = format!("queue-{}", uuid::Uuid::new_v4());
    let item = QueueItem {
        id,
        name,
        max_concurrent,
        is_active: false,
        scheduled_start,
        scheduled_stop,
        power_action,
        items_count: 0,
    };
    let mut lock = state.queues.write().await;
    lock.push(item.clone());
    persist_queues(&lock).await?;
    Ok(item)
}

#[tauri::command]
pub async fn delete_queue(state: tauri::State<'_, QueueState>, id: String) -> Result<(), String> {
    let mut lock = state.queues.write().await;
    lock.retain(|q| q.id != id);
    persist_queues(&lock).await?;
    Ok(())
}
