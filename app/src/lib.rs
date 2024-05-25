pub mod components;
pub mod data;
pub mod tauri_bind {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        // basic tauri invoke function
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
        pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_LOG__"])]
        pub async fn info(args: JsValue) -> JsValue;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_LOG__"])]
        pub async fn warn(args: JsValue) -> JsValue;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_LOG__"])]
        pub async fn error(args: JsValue) -> JsValue;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_LOG__"])]
        pub async fn attachConsole() -> JsValue;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_BARCODESCANNER__"])]
        pub async fn scan(args: JsValue) -> JsValue;
    }
}

pub mod js_bind {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/public/index.bundle.mjs")]
    extern "C" {
        #[wasm_bindgen(js_name = "captureAndPredict")]
        pub fn capture_and_predict();

        #[wasm_bindgen(js_name = "echo")]
        pub fn echo(s: &str);

        #[wasm_bindgen(js_name = "initVideoProcess", catch)]
        pub async fn init_video_process() -> Result<(), JsValue>;

        #[wasm_bindgen(js_name = "onTriggerUpload")]
        pub fn on_trigger_upload();
    }
}
