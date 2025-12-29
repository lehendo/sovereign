#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sovereign_lib::{commands, AppState, Database, ScreenRecorder};
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::env;
use tauri::Manager;

#[cfg(target_os = "windows")]
fn setup_tesseract_windows(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;
    
    // Get the resource directory where Tesseract is bundled
    let resource_dir = app_handle
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;
    
    let tesseract_dir = resource_dir.join("tesseract-win");
    let tesseract_exe = tesseract_dir.join("tesseract.exe");
    let tessdata_dir = tesseract_dir.join("tessdata");
    
    // Check if bundled Tesseract exists
    if !tesseract_exe.exists() {
        return Err(format!(
            "Bundled Tesseract not found at: {}\nPlease ensure the Windows build includes Tesseract resources.",
            tesseract_dir.display()
        ).into());
    }
    
    // Add Tesseract directory to PATH for this process
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", tesseract_dir.display(), current_path);
    env::set_var("PATH", &new_path);
    
    // Set TESSDATA_PREFIX environment variable
    if tessdata_dir.exists() {
        if let Some(tessdata_path) = tessdata_dir.to_str() {
            env::set_var("TESSDATA_PREFIX", tessdata_path);
            println!("Set TESSDATA_PREFIX to: {}", tessdata_dir.display());
        } else {
            eprintln!("Warning: tessdata directory path contains invalid UTF-8: {}", tessdata_dir.display());
        }
    } else {
        eprintln!("Warning: tessdata directory not found at: {}", tessdata_dir.display());
    }
    
    println!("Windows Tesseract setup complete:");
    println!("  Tesseract path: {}", tesseract_exe.display());
    println!("  Added to PATH: {}", tesseract_dir.display());
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn check_tesseract_installed() -> Result<(), String> {
    // On macOS, check common Homebrew installation paths first
    #[cfg(target_os = "macos")]
    {
        let homebrew_paths = vec![
            "/opt/homebrew/bin/tesseract",  // Apple Silicon Homebrew
            "/usr/local/bin/tesseract",      // Intel Homebrew
            "/opt/homebrew/opt/tesseract/bin/tesseract",  // Alternative Homebrew location
        ];
        
        for path in homebrew_paths {
            if std::path::Path::new(path).exists() {
                match Command::new(path).arg("--version").output() {
                    Ok(result) if result.status.success() => {
                        let version = String::from_utf8_lossy(&result.stdout);
                        println!("Tesseract found at {}: {}", path, version.lines().next().unwrap_or("unknown"));
                        return Ok(());
                    }
                    _ => continue,
                }
            }
        }
    }
    
    // Fallback: Check if tesseract command is available in PATH
    let output = Command::new("tesseract")
        .arg("--version")
        .output();
    
    match output {
        Ok(result) if result.status.success() => {
            let version = String::from_utf8_lossy(&result.stdout);
            println!("Tesseract found in PATH: {}", version.lines().next().unwrap_or("unknown"));
            Ok(())
        }
        Ok(_) => Err("Tesseract command failed".to_string()),
        Err(_) => Err("Tesseract not found. Please install it using: brew install tesseract".to_string()),
    }
}

#[cfg(not(target_os = "windows"))]
fn show_tesseract_error_dialog() {
    use std::process;
    
    let os_name = if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "Linux"
    };
    
    let install_command = if cfg!(target_os = "macos") {
        "brew install tesseract"
    } else {
        "sudo apt install tesseract-ocr"
    };
    
    // Try to show a macOS dialog if available
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            r#"osascript -e 'display dialog "Sovereign requires Tesseract OCR to be installed.\n\nPlease install it by running:\n\n  brew install tesseract\n\nAfter installation, restart the application." buttons {{"OK"}} default button "OK" with icon stop with title "Tesseract OCR Required"'"#
        );
        let _ = Command::new("sh").arg("-c").arg(&script).output();
    }
    
    eprintln!("═══════════════════════════════════════════════════════════");
    eprintln!("  DEPENDENCY MISSING: Tesseract OCR");
    eprintln!("═══════════════════════════════════════════════════════════");
    eprintln!();
    eprintln!("Tesseract OCR is required for text extraction.");
    eprintln!("Please install it using:");
    eprintln!();
    eprintln!("  {}: {}", os_name, install_command);
    eprintln!();
    eprintln!("After installation, restart the application.");
    eprintln!("═══════════════════════════════════════════════════════════");
    
    // Exit gracefully
    process::exit(1);
}

fn setup_tesseract(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        setup_tesseract_windows(app_handle)?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        match check_tesseract_installed() {
            Ok(_) => {
                println!("Tesseract OCR check passed");
            }
            Err(e) => {
                eprintln!("Tesseract OCR check failed: {}", e);
                show_tesseract_error_dialog();
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Setup Tesseract (Windows: bundle, macOS/Linux: check)
            if let Err(e) = setup_tesseract(&app_handle) {
                eprintln!("FATAL: Tesseract setup failed: {:#}", e);
                return Err(e);
            }
            
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .map_err(|e| {
                    eprintln!("FATAL: Failed to get app data directory: {:#}", e);
                    format!("Failed to get app data directory: {}", e)
                })?;
            
            println!("App data directory location: {}", app_data_dir.display());
            
            // Ensure the app data directory exists
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir)
                    .map_err(|e| {
                        eprintln!("FATAL: Failed to create app data directory: {:#}", e);
                        format!("Failed to create app data directory: {}", e)
                    })?;
            }
            
            let db_path = app_data_dir.join("sovereign.db");

            let database = Database::new(db_path)
                .map_err(|e| {
                    eprintln!("FATAL: Failed to initialize database: {:#}", e);
                    e
                })?;

            println!("Running retention policy check...");
            match database.prune_old_data(14) {
                Ok(count) if count > 0 => {
                    println!("Retention policy: Removed {} old frames", count);
                },
                Ok(_) => println!("Retention policy: No old frames to remove"),
                Err(e) => eprintln!("Warning: Failed to prune old data: {}", e),
            }

            let embedding_model = ScreenRecorder::load_embedding_model_offline()
                .unwrap_or_else(|e| {
                    eprintln!("Error loading embedding model: {:#}", e);
                    eprintln!("Search will not be available without embeddings");
                    None
                });

            let app_state = AppState {
                database: Arc::new(Mutex::new(database)),
                embedding_model: Arc::new(Mutex::new(embedding_model)),
            };

            app.manage(app_state);
            
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
            commands::get_frames_from_past_days,
            commands::get_database_stats,
            commands::read_image_file,
        ])
        .run(tauri::generate_context!())
        .map_err(|e| {
            eprintln!("FATAL: Failed to run Tauri application: {:#}", e);
            e
        })?;
        
        Ok(())
}

