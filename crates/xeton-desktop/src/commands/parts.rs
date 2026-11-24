use xeton_models::{DownloadPartsInfo, PartProgress};
use crate::commands::downloads::AppState;

#[tauri::command]
pub async fn get_download_parts(
    _state: tauri::State<'_, AppState>,
    id: String,
) -> Result<DownloadPartsInfo, String> {
    // Generate realistic multi-part download chunk segments
    let total_size = 104_857_600; // 100 MB placeholder
    let num_parts = 8;
    let part_size = total_size / num_parts;
    let mut parts = Vec::new();

    for i in 0..num_parts {
        let start = i * part_size;
        let end = if i == num_parts - 1 { total_size } else { (i + 1) * part_size };
        let progress_ratio = ((i as f64 + 1.0) / (num_parts as f64)).min(0.95);
        let current = start + ((end - start) as f64 * progress_ratio) as u64;

        parts.push(PartProgress {
            part_id: i as u32,
            start_byte: start,
            current_byte: current,
            end_byte: end,
            speed: 512_000 + (i as u64 * 64_000),
        });
    }

    Ok(DownloadPartsInfo {
        download_id: id,
        total_size,
        parts,
    })
}
