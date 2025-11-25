// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sovereign_lib::ScreenRecorder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Spawn the screen recorder task
            tauri::async_runtime::spawn(async move {
                let recorder = ScreenRecorder::new(app_handle);
                println!("Starting screen capture loop...");
                recorder.start_capture_loop().await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

