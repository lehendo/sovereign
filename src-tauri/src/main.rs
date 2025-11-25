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
                match ScreenRecorder::new(app_handle) {
                    Ok(recorder) => {
                        println!("Starting screen capture loop with OCR and embeddings...");
                        recorder.start_capture_loop().await;
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize ScreenRecorder: {:#}", e);
                        eprintln!("Make sure Tesseract is installed on your system");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

