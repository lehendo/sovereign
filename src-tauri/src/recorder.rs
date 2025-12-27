use anyhow::{Context, Result};
use fastembed::{TextEmbedding, UserDefinedEmbeddingModel};
use image::{DynamicImage, GenericImageView};
use image_hasher::{HasherConfig, ImageHash};
use rusty_tesseract::{Args, Image as TesseractImage};
use tempfile::NamedTempFile;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tauri::Manager;
use xcap::Monitor;

const VERBOSE_LOGGING: bool = cfg!(debug_assertions);

use crate::database::Database;

pub struct ScreenRecorder {
    last_hash: Option<ImageHash>,
    hasher: image_hasher::Hasher,
    embedding_model: Option<TextEmbedding>,
    database: Database,
    app_handle: AppHandle,
    blacklist: Vec<String>,
}

impl ScreenRecorder {
    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn embedding_model(&self) -> Option<&TextEmbedding> {
        self.embedding_model.as_ref()
    }
}

impl ScreenRecorder {
    pub fn load_embedding_model_offline() -> Result<Option<TextEmbedding>> {
        println!("Initializing embedding model from local cache...");
        
        let possible_cache_dirs = vec![
            dirs::cache_dir().map(|c| c.join("huggingface")),
            dirs::home_dir().map(|h| h.join(".cache").join("huggingface")),
        ];

        let mut cache_dir = None;
        for possible_dir in possible_cache_dirs.into_iter().flatten() {
            let model_dir = possible_dir
                .join("hub")
                .join("models--Qdrant--all-MiniLM-L6-v2-onnx")
                .join("snapshots")
                .join("main");
            
            if model_dir.exists() {
                cache_dir = Some(model_dir);
                break;
            }
        }

        let cache_dir = match cache_dir {
            Some(dir) => dir,
            None => {
                println!("Model cache not found in standard locations");
                println!("Checked:");
                if cfg!(target_os = "windows") {
                    println!("  - %LOCALAPPDATA%\\huggingface\\hub\\models--Qdrant--all-MiniLM-L6-v2-onnx\\snapshots\\main");
                } else if cfg!(target_os = "macos") {
                    println!("  - ~/Library/Caches/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main");
                    println!("  - ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main");
                } else {
                    println!("  - ~/.cache/huggingface/hub/models--Qdrant--all-MiniLM-L6-v2-onnx/snapshots/main");
                }
                println!("To enable embeddings, manually download model files from:");
                println!("https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx");
                println!("See README.md for platform-specific instructions");
                println!("Running in OCR-only mode");
                return Ok(None);
            }
        };

        let model_path = cache_dir.join("model.onnx");
        let tokenizer_path = cache_dir.join("tokenizer.json");
        let config_path = cache_dir.join("config.json");
        let special_tokens_path = cache_dir.join("special_tokens_map.json");
        let tokenizer_config_path = cache_dir.join("tokenizer_config.json");

        if !model_path.exists() || !tokenizer_path.exists() || !config_path.exists() 
            || !special_tokens_path.exists() || !tokenizer_config_path.exists() {
            eprintln!("Warning: Missing model files in cache directory");
            eprintln!("Expected files: model.onnx, tokenizer.json, config.json, special_tokens_map.json, tokenizer_config.json");
            eprintln!("Cache directory: {}", cache_dir.display());
            println!("Running in OCR-only mode");
            return Ok(None);
        }

        println!("Loading model files from: {}", cache_dir.display());
        let model_bytes = std::fs::read(&model_path)
            .context("Failed to read model.onnx")?;
        let tokenizer_bytes = std::fs::read(&tokenizer_path)
            .context("Failed to read tokenizer.json")?;
        let config_bytes = std::fs::read(&config_path)
            .context("Failed to read config.json")?;
        let special_tokens_bytes = std::fs::read(&special_tokens_path)
            .context("Failed to read special_tokens_map.json")?;
        let tokenizer_config_bytes = std::fs::read(&tokenizer_config_path)
            .context("Failed to read tokenizer_config.json")?;

        println!("Model files loaded successfully:");
        println!("  - model.onnx: {:.2} MB", model_bytes.len() as f64 / 1_000_000.0);
        println!("  - tokenizer.json: {:.2} KB", tokenizer_bytes.len() as f64 / 1_000.0);
        println!("  - config.json: {} bytes", config_bytes.len());
        println!("  - special_tokens_map.json: {} bytes", special_tokens_bytes.len());
        println!("  - tokenizer_config.json: {} bytes", tokenizer_config_bytes.len());

        let tokenizer_files = fastembed::TokenizerFiles {
            tokenizer_file: tokenizer_bytes,
            config_file: config_bytes,
            special_tokens_map_file: special_tokens_bytes,
            tokenizer_config_file: tokenizer_config_bytes,
        };

        let user_model = UserDefinedEmbeddingModel::new(model_bytes, tokenizer_files);

        match TextEmbedding::try_new_from_user_defined(user_model, Default::default()) {
            Ok(model) => {
                println!("Embedding model initialized successfully (offline mode)");
                Ok(Some(model))
            }
            Err(e) => {
                eprintln!("Warning: Failed to initialize embedding model: {:#}", e);
                eprintln!("Running in OCR-only mode");
                Ok(None)
            }
        }
    }

    pub fn new(app_handle: AppHandle) -> Result<Self> {
        let hasher = HasherConfig::new()
            .hash_size(16, 16)
            .preproc_dct()
            .to_hasher();

        let db_path = app_handle
            .path()
            .app_data_dir()
            .context("Failed to get app data directory")?
            .join("sovereign.db");

        let database = Database::new(db_path)?;

        let embedding_model = Self::load_embedding_model_offline()
            .unwrap_or_else(|e| {
                eprintln!("Error loading embedding model: {:#}", e);
                eprintln!("Continuing in OCR-only mode");
                None
            });

        let blacklist = vec![
            "Incognito".to_string(),
            "Tor Browser".to_string(),
            "Bitwarden".to_string(),
            "1Password".to_string(),
            "KeePass".to_string(),
            "LastPass".to_string(),
            "InPrivate".to_string(),
            "Private Browsing".to_string(),
            "Private Window".to_string(),
            "New Incognito Window".to_string(),
            "Incognito Window".to_string(),
            "Private Tab".to_string(),
            "Private Mode".to_string(),
        ];

        Ok(Self {
            last_hash: None,
            hasher,
            embedding_model,
            database,
            app_handle,
            blacklist,
        })
    }

    // Privacy Guard temporarily disabled - commented out for future use
    /*fn check_privacy_guard(&self) -> bool {
        let window_title = self.get_active_window_title();
        
        match window_title {
            Some(title) => {
                let title_lower = title.to_lowercase();
                
                let browser_indicators = vec!["safari", "chrome", "firefox", "edge", "brave", "opera", "browser"];
                let is_browser = browser_indicators.iter().any(|browser| {
                    title_lower.contains(browser)
                });
                
                if is_browser {
                    if let Ok(is_private) = self.check_browser_private_mode(&title_lower) {
                        if is_private {
                            println!("Privacy Guard triggered: Detected private/incognito mode via OCR");
                            println!("Skipping capture for: {}", title);
                            return true;
                        }
                    }
                }
                
                for blocked_term in &self.blacklist {
                    let term_lower = blocked_term.to_lowercase();
                    if title_lower.contains(&term_lower) {
                        println!("Privacy Guard triggered: Window title contains '{}'", blocked_term);
                        println!("Skipping capture for: {}", title);
                        return true;
                    }
                }
                
                false
            }
            None => {
                if VERBOSE_LOGGING {
                    eprintln!("[Privacy Guard] Could not detect active window - allowing capture");
                }
                false
            }
        }
    }*/
    
    // Privacy Guard temporarily disabled - commented out for future use
    /*fn check_browser_private_mode(&self, _app_name: &str) -> Result<bool> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let browser_processes = vec!["Safari", "Google Chrome", "Chrome", "Firefox", "Microsoft Edge", "Brave Browser", "Opera"];
            
            let monitors = Monitor::all().context("Failed to get monitors for OCR check")?;
            let primary_monitor = monitors.into_iter()
                .find(|m| m.is_primary())
                .context("No primary monitor found")?;
            
            let buffer = primary_monitor
                .capture_image()
                .context("Failed to capture screen for OCR check")?;
            
            let full_img = DynamicImage::ImageRgba8(buffer);
            let (monitor_x, monitor_y) = (primary_monitor.x(), primary_monitor.y());
            
            for proc_name in &browser_processes {
                let process_exists = Command::new("osascript")
                    .arg("-e")
                    .arg(format!(r#"tell application "System Events" to exists process "{}""#, proc_name))
                    .output();
                
                let process_running = if let Ok(exists_output) = process_exists {
                    exists_output.status.success() && 
                    String::from_utf8_lossy(&exists_output.stdout).trim() == "true"
                } else {
                    false
                };
                
                if !process_running {
                    continue;
                }
                
                if *proc_name == "Safari" {
                    let all_windows_script = r#"tell application "Safari" to get private browsing enabled of every window"#;
                    let all_windows_check = Command::new("osascript")
                        .arg("-e")
                        .arg(all_windows_script)
                        .output();
                    
                    if let Ok(all_output) = all_windows_check {
                        if all_output.status.success() {
                            let all_private_str = String::from_utf8_lossy(&all_output.stdout);
                            if all_private_str.contains("true") {
                                if VERBOSE_LOGGING {
                                    println!("[Privacy Guard] Safari API detected private browsing in any window");
                                }
                                return Ok(true);
                            }
                        }
                    }
                }
                
                let windows_count_script = format!(
                    r#"tell application "System Events" to tell process "{}" to get count of windows"#,
                    proc_name
                );
                
                let window_count_output = Command::new("osascript")
                    .arg("-e")
                    .arg(&windows_count_script)
                    .output();
                
                let window_count = if let Ok(count_output) = window_count_output {
                    if count_output.status.success() {
                        String::from_utf8_lossy(&count_output.stdout).trim().parse::<usize>().unwrap_or(0)
                    } else {
                        0
                    }
                } else {
                    0
                };
                
                if window_count > 0 {
                    for window_index in 1..=window_count {
                        let bounds_script = format!(
                            r#"tell application "System Events" to tell process "{}" to get bounds of window {}"#,
                            proc_name, window_index
                        );
                        
                        let bounds_output = Command::new("osascript")
                            .arg("-e")
                            .arg(&bounds_script)
                            .output();
                        
                        if let Ok(bounds_result) = bounds_output {
                            if bounds_result.status.success() {
                                let bounds_str = String::from_utf8_lossy(&bounds_result.stdout).trim().to_string();
                                let bounds_parts: Vec<&str> = bounds_str.split(", ").collect();
                                
                                if bounds_parts.len() == 4 {
                                    if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
                                        bounds_parts[0].parse::<i32>(),
                                        bounds_parts[1].parse::<i32>(),
                                        bounds_parts[2].parse::<i32>(),
                                        bounds_parts[3].parse::<i32>(),
                                    ) {
                                        let window_x_on_screen = x - monitor_x;
                                        let window_y_on_screen = y - monitor_y;
                                        
                                        if window_x_on_screen >= 0 && window_y_on_screen >= 0 {
                                            let window_x_u32 = window_x_on_screen as u32;
                                            let window_y_u32 = window_y_on_screen as u32;
                                            let w_u32 = w as u32;
                                            let h_u32 = h as u32;
                                            
                                            let max_crop_width = full_img.width().saturating_sub(window_x_u32);
                                            let max_crop_height = full_img.height().saturating_sub(window_y_u32);
                                            
                                            let crop_width = w_u32.min(800).min(max_crop_width);
                                            let crop_height = h_u32.min(80).min(max_crop_height);
                                            
                                            if crop_width > 0 && crop_height > 0 && 
                                               window_x_u32 < full_img.width() && 
                                               window_y_u32 < full_img.height() {
                                                
                                                let cropped = full_img.crop_imm(
                                                    window_x_u32,
                                                    window_y_u32,
                                                    crop_width,
                                                    crop_height,
                                                );
                                                
                                                match self.extract_text_from_image(&cropped) {
                                                    Ok(text) => {
                                                        let text_lower = text.to_lowercase();
                                                        let private_indicators = vec!["incognito", "private", "inprivate"];
                                                        for indicator in private_indicators {
                                                            if text_lower.contains(indicator) {
                                                                if VERBOSE_LOGGING {
                                                                    println!("[Privacy Guard] OCR detected '{}' in browser window {} of {} (bounds: {}, {}, {}, {})", indicator, window_index, window_count, x, y, w, h);
                                                                }
                                                                return Ok(true);
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        if VERBOSE_LOGGING {
                                                            eprintln!("[Privacy Guard] OCR check failed for window {} at ({}, {}): {}", window_index, x, y, e);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            let monitors = Monitor::all().context("Failed to get monitors for OCR check")?;
            let primary_monitor = monitors.into_iter()
                .find(|m| m.is_primary())
                .context("No primary monitor found")?;
            
            let buffer = primary_monitor
                .capture_image()
                .context("Failed to capture screen for OCR check")?;
            
            let full_img = DynamicImage::ImageRgba8(buffer);
            let (width, height) = full_img.dimensions();
            
            let crop_width = width.min(800);
            let crop_height = 80.min(height);
            
            let cropped = full_img.crop_imm(0, 0, crop_width, crop_height);
            
            match self.extract_text_from_image(&cropped) {
                Ok(text) => {
                    let text_lower = text.to_lowercase();
                    let private_indicators = vec!["incognito", "private", "inprivate"];
                    for indicator in private_indicators {
                        if text_lower.contains(indicator) {
                            if VERBOSE_LOGGING {
                                println!("[Privacy Guard] OCR detected '{}' in top region", indicator);
                            }
                            return Ok(true);
                        }
                    }
                }
                Err(e) => {
                    if VERBOSE_LOGGING {
                        eprintln!("[Privacy Guard] OCR check failed: {}", e);
                    }
                }
            }
        }
        
        Ok(false)
    }*/
    
    // Privacy Guard temporarily disabled - commented out for future use
    /*fn get_active_window_title(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("osascript")
                .arg("-e")
                .arg(r#"tell application "System Events" to get name of first process whose frontmost is true"#)
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    let app_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    
                    let sanitized_app_name = app_name
                        .replace('"', "")
                        .replace('\n', "")
                        .replace('\r', "")
                        .replace('\\', "");
                    
                    let app_name_lower = app_name.to_lowercase();
                    
                    if app_name_lower.contains("safari") {
                        let private_check = Command::new("osascript")
                            .arg("-e")
                            .arg(r#"tell application "Safari" to get private browsing enabled of front window"#)
                            .output();
                        
                        if let Ok(private_output) = private_check {
                            if private_output.status.success() {
                                let private_str = String::from_utf8_lossy(&private_output.stdout);
                                let is_private = private_str.trim();
                                if is_private == "true" {
                                    if VERBOSE_LOGGING {
                                        println!("[Privacy Guard] Detected Safari private browsing window via Safari API");
                                    }
                                    return Some("Safari - Private Browsing".to_string());
                                }
                            } else {
                                if VERBOSE_LOGGING {
                                    let error = String::from_utf8_lossy(&private_output.stderr);
                                    println!("[Privacy Guard] Safari automation permission denied or unavailable: {}", error);
                                    println!("[Privacy Guard] Falling back to window title detection");
                                }
                            }
                        } else {
                            if VERBOSE_LOGGING {
                                println!("[Privacy Guard] Failed to check Safari private browsing status, falling back to window title detection");
                            }
                        }
                    }
                    
                    let window_output = Command::new("osascript")
                        .arg("-e")
                        .arg(format!(
                            r#"tell application "System Events" to tell process "{}" to get title of front window"#,
                            sanitized_app_name
                        ))
                        .output();
                    
                    let window_title = if let Ok(window_output) = window_output {
                        if window_output.status.success() {
                            Some(String::from_utf8_lossy(&window_output.stdout).trim().to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    
                    if let Some(ref win_title) = window_title {
                        let win_title_lower = win_title.to_lowercase();
                        
                        if app_name_lower.contains("chrome") || app_name_lower.contains("google chrome") {
                            println!("[Privacy Guard] Checking Chrome window title: '{}'", win_title);
                            if win_title_lower.contains("incognito") {
                                println!("[Privacy Guard] Detected Chrome incognito window via window title: {}", win_title);
                                return Some("Google Chrome - Incognito".to_string());
                            }
                            
                            let chrome_incognito_check = Command::new("osascript")
                                .arg("-e")
                                .arg(format!(
                                    r#"tell application "System Events" to tell process "{}" to get name of front window"#,
                                    sanitized_app_name
                                ))
                                .output();
                            
                            if let Ok(chrome_output) = chrome_incognito_check {
                                if chrome_output.status.success() {
                                    let chrome_name = String::from_utf8_lossy(&chrome_output.stdout).trim().to_lowercase();
                                    if chrome_name.contains("incognito") || chrome_name.contains("(incognito)") {
                                        if VERBOSE_LOGGING {
                                            println!("[Privacy Guard] Detected Chrome incognito window via window name: {}", chrome_name);
                                        }
                                        return Some("Google Chrome - Incognito".to_string());
                                    }
                                }
                            }
                        }
                        
                        let full_title = format!("{} - {}", app_name, win_title);
                        println!("[Privacy Guard] Window title: {}", full_title);
                        if VERBOSE_LOGGING {
                            println!("[Privacy Guard] Detected window: {}", full_title);
                        }
                        return Some(full_title);
                    } else {
                        if VERBOSE_LOGGING {
                            println!("[Privacy Guard] Detected app: {} (window title unavailable)", app_name);
                        }
                        return Some(app_name);
                    }
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    eprintln!("[Privacy Guard] Failed to get active window: {}", error);
                    eprintln!("[Privacy Guard] This usually means Accessibility permission is not granted");
                    eprintln!("[Privacy Guard] Go to: System Settings > Privacy & Security > Accessibility");
                }
            } else {
                eprintln!("[Privacy Guard] Failed to execute osascript command");
            }
            
            None
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            let output = Command::new("powershell")
                .arg("-Command")
                .arg("Add-Type @\"\nusing System;\nusing System.Runtime.InteropServices;\nusing System.Text;\npublic class WindowTitle {\n    [DllImport(\"user32.dll\")]\n    static extern IntPtr GetForegroundWindow();\n    [DllImport(\"user32.dll\")]\n    static extern int GetWindowText(IntPtr hWnd, StringBuilder text, int count);\n    public static string GetActiveWindowTitle() {\n        const int nChars = 256;\n        StringBuilder Buff = new StringBuilder(nChars);\n        IntPtr handle = GetForegroundWindow();\n        if (GetWindowText(handle, Buff, nChars) > 0) return Buff.ToString();\n        return null;\n    }\n}\n\"@; [WindowTitle]::GetActiveWindowTitle()")
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
                }
            }
            
            None
        }
        
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("xdotool")
                .arg("getactivewindow")
                .arg("getwindowname")
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
                }
            }
            
            None
        }
    }*/

    fn get_screenshots_dir(&self) -> Result<PathBuf> {
        let app_data_dir = self
            .app_handle
            .path()
            .app_data_dir()
            .context("Failed to get app data directory")?;

        println!("App data directory: {}", app_data_dir.display());
        
        // Ensure the app data directory exists and is a directory
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)
                .context("Failed to create app data directory")?;
            println!("Created app data directory: {}", app_data_dir.display());
        } else {
            let metadata = std::fs::metadata(&app_data_dir)
                .context("Failed to get app data directory metadata")?;
            if !metadata.is_dir() {
                anyhow::bail!("App data path exists but is not a directory: {}", app_data_dir.display());
            }
        }

        let screenshots_dir = app_data_dir.join("screenshots");

        if !screenshots_dir.exists() {
            std::fs::create_dir_all(&screenshots_dir)
                .context("Failed to create screenshots directory")?;
            println!("Created screenshots directory: {}", screenshots_dir.display());
        }

        Ok(screenshots_dir)
    }

    fn extract_text_from_image(&self, img: &DynamicImage) -> Result<String> {
        let temp_file = NamedTempFile::with_suffix(".png")
            .context("Failed to create temporary file for OCR")?;
        let temp_path = temp_file.path().to_path_buf();

        img.save_with_format(&temp_path, image::ImageFormat::Png)
            .context("Failed to write temporary image for OCR")?;

        let mut args = Args::default();
        args.lang = "eng".to_string();
        args.dpi = Some(300);

        let ocr_image = TesseractImage::from_path(&temp_path)
            .context("Failed to load image for OCR")?;
        
        let text = rusty_tesseract::image_to_string(&ocr_image, &args)
            .context("OCR failed")?;

        std::fs::remove_file(&temp_path)
            .context("Failed to remove OCR temporary file")?;

        Ok(text.trim().to_string())
    }

    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        if text.is_empty() {
            return Ok(vec![]);
        }

        let Some(model) = &self.embedding_model else {
            anyhow::bail!("Embedding model not initialized");
        };

        let embeddings = model
            .embed(vec![text], None)
            .context("Failed to generate embedding")?;

        Ok(embeddings.into_iter().next().unwrap_or_default())
    }

    pub async fn capture_frame(&mut self) -> Result<()> {
        // Privacy Guard temporarily disabled - commented out for future use
        // if self.check_privacy_guard() {
        //     if VERBOSE_LOGGING {
        //         println!("[Privacy Guard] Blocked capture");
        //     }
        //     return Ok(());
        // }

        let monitors = Monitor::all().context("Failed to get monitors")?;
        
        if monitors.is_empty() {
            anyhow::bail!("No monitors found");
        }

        if monitors.len() > 1 {
            println!("Detected {} monitors, capturing all and combining", monitors.len());
        }

        let mut captured_images: Vec<DynamicImage> = Vec::new();
        let mut total_width = 0u32;
        let mut max_height = 0u32;

        for monitor in &monitors {
            println!("Capturing monitor: {} at ({}, {}) - {}x{}", 
                monitor.name(),
                monitor.x(),
                monitor.y(),
                monitor.width(),
                monitor.height()
            );
            
            let buffer = monitor
                .capture_image()
                .context(format!("Failed to capture monitor: {:?}", monitor.name()))?;
            
            let img = DynamicImage::ImageRgba8(buffer);
            let (width, height) = img.dimensions();
            println!("Successfully captured monitor: {}x{} pixels", width, height);
            total_width += width;
            max_height = max_height.max(height);
            captured_images.push(img);
        }

        let combined_img = if captured_images.len() == 1 {
            // Safe unwrap: we just checked len() == 1
            captured_images.into_iter().next().expect("Expected exactly one image")
        } else {
            let mut canvas = image::RgbaImage::new(total_width, max_height);
            let mut x_offset = 0u32;
            
            for img in captured_images {
                let (width, _height) = img.dimensions();
                image::imageops::overlay(&mut canvas, &img.to_rgba8(), x_offset as i64, 0);
                x_offset += width;
            }
            
            DynamicImage::ImageRgba8(canvas)
        };

        let img = {
            let (width, height) = combined_img.dimensions();
            
            if width > 7680 || height > 4320 {
                let aspect_ratio = width as f32 / height as f32;
                let (new_width, new_height) = if aspect_ratio > (7680.0 / 4320.0) {
                    (7680, (7680.0 / aspect_ratio) as u32)
                } else {
                    ((4320.0 * aspect_ratio) as u32, 4320)
                };
                println!("Resized large image from {}x{} to {}x{}", width, height, new_width, new_height);
                combined_img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
            } else {
                combined_img
            }
        };

        let hash = self.hasher.hash_image(&img);

        if let Some(last_hash) = &self.last_hash {
            let distance = hash.dist(last_hash);
            if distance < 1 {
                if VERBOSE_LOGGING {
                    println!("No change detected (hash distance: {})", distance);
                }
                return Ok(());
            }
        }

        let hash_string = format!("{}", hash.to_base64());

        self.last_hash = Some(hash);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs();

        let filename = format!("{}.webp", timestamp);
        let screenshots_dir = self.get_screenshots_dir()?;
        let filepath = screenshots_dir.join(&filename);

        let rgb_img = img.to_rgb8();

        let encoder = webp::Encoder::from_rgb(&rgb_img, rgb_img.width(), rgb_img.height());
        let webp_data = encoder.encode(95.0);

        std::fs::write(&filepath, &*webp_data)
            .context("Failed to write WebP file")?;

        println!(
            "Captured: {} ({}x{}) -> {}",
            filename,
            img.width(),
            img.height(),
            filepath.display()
        );

        let frame_id = self.database.insert_frame(
            timestamp,
            filepath.to_str().unwrap_or(&filename),
            &hash_string,
            None,
            None,
        ).context("Failed to insert frame into database")?;

        println!("Frame saved to database (ID: {})", frame_id);

        if VERBOSE_LOGGING {
            println!("Performing OCR...");
        }
        let ocr_text = match self.extract_text_from_image(&img) {
            Ok(text) => {
                if VERBOSE_LOGGING {
                    println!("OCR extracted {} characters", text.len());
                }
                text
            }
            Err(e) => {
                eprintln!("OCR error: {:#}", e);
                String::new()
            }
        };

        if !ocr_text.is_empty() {
            if let Err(e) = self.database.insert_ocr_text(frame_id, &ocr_text) {
                eprintln!("Failed to insert OCR text: {:#}", e);
            } else {
                if VERBOSE_LOGGING {
                    println!("OCR text saved to database");
                }
            }
        }

        if !ocr_text.is_empty() {
            if self.embedding_model.is_some() {
                if VERBOSE_LOGGING {
                    println!("Generating embedding...");
                }
                match self.generate_embedding(&ocr_text) {
                    Ok(embedding) => {
                        if VERBOSE_LOGGING {
                            println!("Embedding vector length: {}", embedding.len());
                            println!("First 5 dimensions: {:?}", &embedding[..5.min(embedding.len())]);
                        }
                        
                        if let Err(e) = self.database.insert_embedding(frame_id, &embedding) {
                            eprintln!("Failed to insert embedding: {:#}", e);
                        } else {
                            if VERBOSE_LOGGING {
                                println!("Embedding saved to database");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Embedding generation error: {:#}", e);
                    }
                }
            } else {
                println!("Embedding model not available - skipping embedding generation");
            }
        }

        // Emit event to frontend that a new frame was captured
        println!("Emitting frame-captured event to frontend...");
        match self.app_handle.emit("frame-captured", ()) {
            Ok(_) => {
                if VERBOSE_LOGGING {
                    println!("Frame-captured event emitted successfully");
                }
            }
            Err(e) => {
                eprintln!("Failed to emit frame-captured event: {}", e);
            }
        }

        Ok(())
    }

    pub async fn start_capture_loop(mut self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));

        if let Ok(stats) = self.database.get_stats() {
            println!("=== Database Statistics ===");
            println!("Total frames: {}", stats.total_frames);
            println!("Total OCR entries: {}", stats.total_ocr_entries);
            println!("Total embeddings: {}", stats.total_embeddings);
            println!("===========================");
        }

        loop {
            interval.tick().await;

            if let Err(e) = self.capture_frame().await {
                eprintln!("Capture error: {:#}", e);
            }
        }
    }
}

