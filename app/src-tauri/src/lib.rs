// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::{webview::WebviewWindowBuilder, WebviewUrl};
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    // let devtools = tauri_plugin_devtools::init();
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(move |app| {
            #[cfg(mobile)]
            app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
            // app.handle().plugin(tauri_plugin_nfc::init())?;
            // app.handle().plugin(tauri_plugin_biometric::init())?;

            // let mut webview_window_builder =
            //     WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
            // let webview = webview_window_builder.build().unwrap();
            // #[cfg(debug_assertions)]
            // webview.open_devtools();

            Ok(())
        });

    // #[cfg(debug_assertions)]
    // app = app.plugin(devtools);

    builder
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
