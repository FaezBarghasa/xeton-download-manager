use tokio::sync::RwLock;
use std::sync::Arc;
use xeton_models::{PowerAction, QueueItem};

#[derive(Default)]
pub struct QueueState {
    pub queues: Arc<RwLock<Vec<QueueItem>>>,
}

impl QueueState {
    pub fn new() -> Self {
        Self {
            queues: Arc::new(RwLock::new(vec![
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
            ])),
        }
    }
}

#[tauri::command]
pub async fn get_queues(
    state: tauri::State<'_, QueueState>,
) -> Result<Vec<QueueItem>, String> {
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
    Ok(item)
}

#[tauri::command]
pub async fn delete_queue(
    state: tauri::State<'_, QueueState>,
    id: String,
) -> Result<(), String> {
    let mut lock = state.queues.write().await;
    lock.retain(|q| q.id != id);
    Ok(())
}
