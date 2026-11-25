use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
export async function raw_invoke(cmd, args) {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
        return await window.__TAURI__.core.invoke(cmd, args);
    }
    console.warn("Tauri invoke called outside of Tauri runtime:", cmd);
    return null;
}

export function raw_minimize() {
    try {
        if (window.__TAURI__ && window.__TAURI__.window) {
            const win = (window.__TAURI__.window.getCurrentWebviewWindow || window.__TAURI__.window.getCurrentWindow)();
            win.minimize();
        }
    } catch (e) {
        console.error(e);
    }
}

export function raw_toggle_maximize() {
    try {
        if (window.__TAURI__ && window.__TAURI__.window) {
            const win = (window.__TAURI__.window.getCurrentWebviewWindow || window.__TAURI__.window.getCurrentWindow)();
            win.toggleMaximize();
        }
    } catch (e) {
        console.error(e);
    }
}

export function raw_close() {
    try {
        if (window.__TAURI__ && window.__TAURI__.window) {
            const win = (window.__TAURI__.window.getCurrentWebviewWindow || window.__TAURI__.window.getCurrentWindow)();
            win.close();
        }
    } catch (e) {
        console.error(e);
    }
}
"#)]
extern "C" {
    fn raw_invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
    fn raw_minimize();
    fn raw_toggle_maximize();
    fn raw_close();
}

#[cfg(target_arch = "wasm32")]
pub async fn invoke<T: serde::de::DeserializeOwned, A: serde::Serialize>(
    cmd: &str,
    args: &A,
) -> Result<T, String> {
    let js_args = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;
    let promise = raw_invoke(cmd, js_args);
    let js_val = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| format!("{:?}", e))?;
    if js_val.is_null() || js_val.is_undefined() {
        return Err("No result from Tauri".into());
    }
    serde_wasm_bindgen::from_value(js_val).map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
pub async fn invoke_no_args<T: serde::de::DeserializeOwned>(cmd: &str) -> Result<T, String> {
    let promise = raw_invoke(cmd, wasm_bindgen::JsValue::NULL);
    let js_val = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| format!("{:?}", e))?;
    if js_val.is_null() || js_val.is_undefined() {
        return Err("No result from Tauri".into());
    }
    serde_wasm_bindgen::from_value(js_val).map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke<T: serde::de::DeserializeOwned, A: serde::Serialize>(
    _cmd: &str,
    _args: &A,
) -> Result<T, String> {
    Err("Not running in wasm".into())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke_no_args<T: serde::de::DeserializeOwned>(_cmd: &str) -> Result<T, String> {
    Err("Not running in wasm".into())
}

pub fn minimize_window() {
    #[cfg(target_arch = "wasm32")]
    raw_minimize();
}

pub fn toggle_maximize_window() {
    #[cfg(target_arch = "wasm32")]
    raw_toggle_maximize();
}

pub fn close_window() {
    #[cfg(target_arch = "wasm32")]
    raw_close();
}
