use crate::commands::downloads::AppState;
use xeton_models::{DownloadPartsInfo, PartProgress};

#[tauri::command]
pub async fn get_download_parts(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<DownloadPartsInfo, String> {
    let numeric_id = id
        .parse::<i64>()
        .map_err(|_| format!("Invalid download id: {id}"))?;

    let ranged_parts = state
        .manager
        .get_download_parts(numeric_id)
        .await
        .map_err(|e| e.to_string())?;

    let mut total_size = 0u64;
    let mut parts = Vec::with_capacity(ranged_parts.len());

    for (idx, p) in ranged_parts.iter().enumerate() {
        let start = p.from.max(0) as u64;
        let current = p.current.max(0) as u64;
        let end = p.to.map(|t| (t + 1).max(0) as u64).unwrap_or(current);
        total_size = total_size.max(end);

        parts.push(PartProgress {
            part_id: idx as u32,
            start_byte: start,
            current_byte: current,
            end_byte: end,
            speed: 0,
        });
    }

    Ok(DownloadPartsInfo {
        download_id: id,
        total_size,
        parts,
    })
}
