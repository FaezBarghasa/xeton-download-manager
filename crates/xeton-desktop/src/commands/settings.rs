use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use xeton_models::FullAppSettings;

fn settings_path() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".local").join("share")
        });
    base.join("xeton-download-manager").join("settings.json")
}

#[derive(Default)]
pub struct SettingsState {
    pub settings: Arc<RwLock<FullAppSettings>>,
}

impl SettingsState {
    pub fn new() -> Self {
        let path = settings_path();
        let initial = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str::<FullAppSettings>(&s).ok())
            .unwrap_or_default();
        Self {
            settings: Arc::new(RwLock::new(initial)),
        }
    }
}

#[tauri::command]
pub async fn get_app_settings(
    state: tauri::State<'_, SettingsState>,
) -> Result<FullAppSettings, String> {
    Ok(state.settings.read().await.clone())
}

#[tauri::command]
pub async fn save_app_settings(
    state: tauri::State<'_, SettingsState>,
    app_state: tauri::State<'_, crate::AppState>,
    settings: FullAppSettings,
) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    tokio::fs::write(&path, json)
        .await
        .map_err(|e| e.to_string())?;

    if settings.speed_limit_enabled && settings.max_download_speed > 0 {
        app_state
            .manager
            .set_global_speed_limit(settings.max_download_speed as i64)
            .await;
    } else {
        app_state.manager.set_global_speed_limit(0).await;
    }

    *state.settings.write().await = settings;
    Ok(())
}
