// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sovereign_lib::{commands, AppState, Database, ScreenRecorder};
use std::sync::{Arc, Mutex};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Initialize database
            let db_path = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory")
                .join("sovereign.db");

            let database = Database::new(db_path)
                .expect("Failed to initialize database");

            // Initialize embedding model (shared between recorder and search)
            let embedding_model = ScreenRecorder::load_embedding_model_offline()
                .unwrap_or_else(|e| {
                    eprintln!("Error loading embedding model: {:#}", e);
                    eprintln!("Search will not be available without embeddings");
                    None
                });

            // Create shared state for Tauri commands
            let app_state = AppState {
                database: Arc::new(Mutex::new(database)),
                embedding_model: Arc::new(Mutex::new(embedding_model)),
            };

            // Store state in Tauri
            app.manage(app_state);
            
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
        .invoke_handler(tauri::generate_handler![
            commands::search_frames,
            commands::get_recent_frames,
            commands::get_database_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

