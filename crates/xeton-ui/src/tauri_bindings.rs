#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn tauri_invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window", "getCurrentWindow()"], js_name = minimize)]
    pub fn minimize_window();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window", "getCurrentWindow()"], js_name = toggleMaximize)]
    pub fn toggle_maximize_window();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window", "getCurrentWindow()"], js_name = close)]
    pub fn close_window();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn minimize_window() {}

#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_maximize_window() {}

#[cfg(not(target_arch = "wasm32"))]
pub fn close_window() {}
