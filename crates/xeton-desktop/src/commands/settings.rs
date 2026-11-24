use std::sync::Arc;
use tokio::sync::RwLock;
use xeton_models::FullAppSettings;

#[derive(Default)]
pub struct SettingsState {
    pub settings: Arc<RwLock<FullAppSettings>>,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(FullAppSettings::default())),
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
    settings: FullAppSettings,
) -> Result<(), String> {
    *state.settings.write().await = settings;
    Ok(())
}
