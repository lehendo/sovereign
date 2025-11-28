use anyhow::{Context, Result};
use fastembed::{TextEmbedding, UserDefinedEmbeddingModel};
use image::{DynamicImage, GenericImageView};
use image_hasher::{HasherConfig, ImageHash};
use rusty_tesseract::{Args, Image as TesseractImage};
use tempfile::NamedTempFile;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
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
            "Private".to_string(),
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

    fn check_privacy_guard(&self) -> bool {
        let window_title = self.get_active_window_title();
        
        match window_title {
            Some(title) => {
                let title_lower = title.to_lowercase();
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
                eprintln!("[Privacy Guard] Could not detect active window - allowing capture");
                false
            }
        }
    }
    
    fn get_active_window_title(&self) -> Option<String> {
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
                    
                    let window_output = Command::new("osascript")
                        .arg("-e")
                        .arg(format!(
                            r#"tell application "System Events" to tell process "{}" to get title of front window"#,
                            sanitized_app_name
                        ))
                        .output();
                    
                    if let Ok(window_output) = window_output {
                        if window_output.status.success() {
                            let window_title = String::from_utf8_lossy(&window_output.stdout).trim().to_string();
                            let full_title = format!("{} - {}", app_name, window_title);
                            println!("[Privacy Guard] Detected window: {}", full_title);
                            return Some(full_title);
                        } else {
                            println!("[Privacy Guard] Detected app: {} (window title unavailable)", app_name);
                            return Some(app_name);
                        }
                    } else {
                        println!("[Privacy Guard] Detected app: {} (window title unavailable)", app_name);
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
    }

    fn get_screenshots_dir(&self) -> Result<PathBuf> {
        let app_data_dir = self
            .app_handle
            .path()
            .app_data_dir()
            .context("Failed to get app data directory")?;

        let screenshots_dir = app_data_dir.join("screenshots");

        if !screenshots_dir.exists() {
            std::fs::create_dir_all(&screenshots_dir)
                .context("Failed to create screenshots directory")?;
        }

        Ok(screenshots_dir)
    }

    fn extract_text_from_image(&self, img: &DynamicImage) -> Result<String> {
        let temp_file = NamedTempFile::new()
            .context("Failed to create temporary file for OCR")?;
        let temp_path = temp_file.into_temp_path();

        img.save(&temp_path)
            .context("Failed to write temporary image for OCR")?;

        let mut args = Args::default();
        args.lang = "eng".to_string();
        args.dpi = Some(300);

        let ocr_image = TesseractImage::from_path(&temp_path)
            .context("Failed to load image for OCR")?;
        
        let text = rusty_tesseract::image_to_string(&ocr_image, &args)
            .context("OCR failed")?;

        temp_path
            .close()
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
        if self.check_privacy_guard() {
            return Ok(());
        }

        let monitors = Monitor::all().context("Failed to get monitors")?;
        let primary_monitor = monitors
            .into_iter()
            .find(|m| m.is_primary())
            .context("No primary monitor found")?;

        let buffer = primary_monitor
            .capture_image()
            .context("Failed to capture screen")?;

        let mut img = DynamicImage::ImageRgba8(buffer);

        let (width, height) = img.dimensions();
        if width > 1920 || height > 1080 {
            let aspect_ratio = width as f32 / height as f32;
            let (new_width, new_height) = if aspect_ratio > (1920.0 / 1080.0) {
                (1920, (1920.0 / aspect_ratio) as u32)
            } else {
                ((1080.0 * aspect_ratio) as u32, 1080)
            };
            img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
        }

        let hash = self.hasher.hash_image(&img);

        if let Some(last_hash) = &self.last_hash {
            let distance = hash.dist(last_hash);
            if distance < 5 {
                println!("No change detected (hash distance: {})", distance);
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
        let webp_data = encoder.encode(85.0);

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

