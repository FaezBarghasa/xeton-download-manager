use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ExtensionInstallResult {
    pub success: bool,
    pub extension_path: String,
    pub chrome_host_manifest: Option<String>,
    pub firefox_host_manifest: Option<String>,
    pub message: String,
}

fn get_native_host_binary_path() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join("xeton-native-host");
            if candidate.exists() {
                return candidate;
            }
        }
    }
    // Fallback to local cargo debug/release or default user location
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let default_path = PathBuf::from(home)
        .join(".local")
        .join("bin")
        .join("xeton-native-host");
    default_path
}

fn get_extension_source_dir() -> PathBuf {
    // 1. Check relative to current exe
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join("extension");
            if candidate.exists() {
                return candidate;
            }
            // Check workspace root if running inside target/debug
            let candidate2 = parent.join("../../../extension");
            if candidate2.exists() {
                if let Ok(canon) = candidate2.canonicalize() {
                    return canon;
                }
            }
        }
    }

    // 2. Check local share directory
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let shared = PathBuf::from(&home)
        .join(".local")
        .join("share")
        .join("xeton-download-manager")
        .join("extension");
    if shared.exists() {
        return shared;
    }

    // 3. Fallback to current working directory extension folder
    PathBuf::from("extension")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("extension"))
}

#[tauri::command]
pub async fn get_extension_info() -> Result<ExtensionInstallResult, String> {
    let ext_dir = get_extension_source_dir();
    let host_bin = get_native_host_binary_path();

    Ok(ExtensionInstallResult {
        success: ext_dir.exists(),
        extension_path: ext_dir.to_string_lossy().to_string(),
        chrome_host_manifest: None,
        firefox_host_manifest: None,
        message: format!("Extension directory located at: {}", ext_dir.display()),
    })
}

#[tauri::command]
pub async fn install_browser_extension() -> Result<ExtensionInstallResult, String> {
    let ext_dir = get_extension_source_dir();
    let host_bin = get_native_host_binary_path();

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let home_path = PathBuf::from(&home);

    let host_bin_str = host_bin.to_string_lossy().to_string();

    let chrome_manifest_json = serde_json::json!({
        "name": "com.xeton.integration",
        "description": "Xeton Download Manager Native Messaging Host",
        "path": host_bin_str,
        "type": "stdio",
        "allowed_origins": [
            "chrome-extension://*",
            "chrome-extension://YOUR_EXTENSION_ID_HERE/"
        ]
    });

    let firefox_manifest_json = serde_json::json!({
        "name": "com.xeton.integration",
        "description": "Xeton Download Manager Native Messaging Host",
        "path": host_bin_str,
        "type": "stdio",
        "allowed_extensions": [
            "xeton-integration@example.com"
        ]
    });

    let chrome_targets = vec![
        home_path.join(".config/google-chrome/NativeMessagingHosts"),
        home_path.join(".config/chromium/NativeMessagingHosts"),
        home_path.join(".config/BraveSoftware/Brave-Browser/NativeMessagingHosts"),
        home_path.join(".config/microsoft-edge/NativeMessagingHosts"),
    ];

    let mut installed_chrome = Vec::new();
    for dir in chrome_targets {
        let _ = tokio::fs::create_dir_all(&dir).await;
        let file = dir.join("com.xeton.integration.json");
        if let Ok(json_str) = serde_json::to_string_pretty(&chrome_manifest_json) {
            if tokio::fs::write(&file, json_str).await.is_ok() {
                installed_chrome.push(file.to_string_lossy().to_string());
            }
        }
    }

    let firefox_dir = home_path.join(".mozilla/native-messaging-hosts");
    let _ = tokio::fs::create_dir_all(&firefox_dir).await;
    let firefox_file = firefox_dir.join("com.xeton.integration.json");
    let mut installed_firefox = None;
    if let Ok(json_str) = serde_json::to_string_pretty(&firefox_manifest_json) {
        if tokio::fs::write(&firefox_file, json_str).await.is_ok() {
            installed_firefox = Some(firefox_file.to_string_lossy().to_string());
        }
    }

    Ok(ExtensionInstallResult {
        success: true,
        extension_path: ext_dir.to_string_lossy().to_string(),
        chrome_host_manifest: installed_chrome.first().cloned(),
        firefox_host_manifest: installed_firefox,
        message: format!(
            "Successfully configured native messaging host manifests. Load unpacked extension from: {}",
            ext_dir.display()
        ),
    })
}

#[tauri::command]
pub async fn open_extension_in_browser(browser: String) -> Result<(), String> {
    let ext_dir = get_extension_source_dir();
    let ext_path_str = ext_dir.to_string_lossy().to_string();

    match browser.to_lowercase().as_str() {
        "chrome" | "google-chrome" => {
            // Open chrome extension management page and load-extension argument if possible
            let _ = tokio::process::Command::new("google-chrome")
                .arg(format!("--load-extension={}", ext_path_str))
                .arg("chrome://extensions")
                .spawn();
        }
        "chromium" => {
            let _ = tokio::process::Command::new("chromium")
                .arg(format!("--load-extension={}", ext_path_str))
                .arg("chrome://extensions")
                .spawn();
        }
        "brave" => {
            let _ = tokio::process::Command::new("brave-browser")
                .arg(format!("--load-extension={}", ext_path_str))
                .arg("brave://extensions")
                .spawn();
        }
        "firefox" => {
            let _ = tokio::process::Command::new("firefox")
                .arg("about:debugging#/runtime/this-firefox")
                .spawn();
        }
        _ => {
            // Open extension directory in file manager
            let _ = tokio::process::Command::new("xdg-open")
                .arg(&ext_path_str)
                .spawn();
        }
    }
    Ok(())
}
