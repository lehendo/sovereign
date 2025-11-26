use anyhow::{Context, Result};
use fastembed::{TextEmbedding, UserDefinedEmbeddingModel};
use image::{DynamicImage, GenericImageView};
use image_hasher::{HasherConfig, ImageHash};
use rusty_tesseract::{Args, Image as TesseractImage};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri::Manager;
use xcap::Monitor;

pub struct ScreenRecorder {
    last_hash: Option<ImageHash>,
    hasher: image_hasher::Hasher,
    embedding_model: Option<TextEmbedding>,
    app_handle: AppHandle,
}

impl ScreenRecorder {
    /// Load embedding model files from cache directory
    fn load_embedding_model_offline() -> Result<Option<TextEmbedding>> {
        println!("Initializing embedding model from local cache...");
        
        // Try multiple cache locations (cross-platform)
        let possible_cache_dirs = vec![
            // Windows: %LOCALAPPDATA%\huggingface (C:\Users\Name\AppData\Local\huggingface)
            // macOS: ~/Library/Caches/huggingface
            // Linux: ~/.cache/huggingface
            dirs::cache_dir().map(|c| c.join("huggingface")),
            // Fallback for Linux if cache_dir doesn't work
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

        // Load required model files
        let model_path = cache_dir.join("model.onnx");
        let tokenizer_path = cache_dir.join("tokenizer.json");
        let config_path = cache_dir.join("config.json");
        let special_tokens_path = cache_dir.join("special_tokens_map.json");
        let tokenizer_config_path = cache_dir.join("tokenizer_config.json");

        // Check if all required files exist
        if !model_path.exists() || !tokenizer_path.exists() || !config_path.exists() 
            || !special_tokens_path.exists() || !tokenizer_config_path.exists() {
            eprintln!("Warning: Missing model files in cache directory");
            eprintln!("Expected files: model.onnx, tokenizer.json, config.json, special_tokens_map.json, tokenizer_config.json");
            eprintln!("Cache directory: {}", cache_dir.display());
            println!("Running in OCR-only mode");
            return Ok(None);
        }

        // Read files into memory
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

        // Construct TokenizerFiles
        let tokenizer_files = fastembed::TokenizerFiles {
            tokenizer_file: tokenizer_bytes,
            config_file: config_bytes,
            special_tokens_map_file: special_tokens_bytes,
            tokenizer_config_file: tokenizer_config_bytes,
        };

        // Construct UserDefinedEmbeddingModel using the constructor
        let user_model = UserDefinedEmbeddingModel::new(model_bytes, tokenizer_files);

        // Initialize TextEmbedding from user-defined model
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

    /// Create a new ScreenRecorder instance
    pub fn new(app_handle: AppHandle) -> Result<Self> {
        let hasher = HasherConfig::new()
            .hash_size(16, 16)
            .preproc_dct()
            .to_hasher();

        // Load embedding model from cache (offline)
        let embedding_model = Self::load_embedding_model_offline()
            .unwrap_or_else(|e| {
                eprintln!("Error loading embedding model: {:#}", e);
                eprintln!("Continuing in OCR-only mode");
                None
            });

        Ok(Self {
            last_hash: None,
            hasher,
            embedding_model,
            app_handle,
        })
    }

    /// Get the screenshots directory path using Tauri's AppData
    fn get_screenshots_dir(&self) -> Result<PathBuf> {
        let app_data_dir = self
            .app_handle
            .path()
            .app_data_dir()
            .context("Failed to get app data directory")?;

        let screenshots_dir = app_data_dir.join("screenshots");

        // Create directory if it doesn't exist
        if !screenshots_dir.exists() {
            std::fs::create_dir_all(&screenshots_dir)
                .context("Failed to create screenshots directory")?;
        }

        Ok(screenshots_dir)
    }

    /// Extract text from image using OCR
    fn extract_text_from_image(&self, img: &DynamicImage) -> Result<String> {
        // Save image temporarily for Tesseract
        let temp_path = std::env::temp_dir().join("sovereign_ocr_temp.png");
        img.save(&temp_path)
            .context("Failed to save temporary image for OCR")?;

        // Configure Tesseract
        let mut args = Args::default();
        args.lang = "eng".to_string();
        args.dpi = Some(300);

        // Perform OCR
        let ocr_image = TesseractImage::from_path(&temp_path)
            .context("Failed to load image for OCR")?;
        
        let text = rusty_tesseract::image_to_string(&ocr_image, &args)
            .context("OCR failed")?;

        // Clean up temp file
        let _ = std::fs::remove_file(temp_path);

        Ok(text.trim().to_string())
    }

    /// Generate embedding vector from text
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

    /// Capture a single frame from the primary monitor
    pub async fn capture_frame(&mut self) -> Result<()> {
        // Get all monitors and select the primary one
        let monitors = Monitor::all().context("Failed to get monitors")?;
        let primary_monitor = monitors
            .into_iter()
            .find(|m| m.is_primary())
            .context("No primary monitor found")?;

        // Capture the screen
        let buffer = primary_monitor
            .capture_image()
            .context("Failed to capture screen")?;

        // Convert to DynamicImage
        let mut img = DynamicImage::ImageRgba8(buffer);

        // Resize to 1080p if larger (Step 1.4)
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

        // Calculate perceptual hash
        let hash = self.hasher.hash_image(&img);

        // Check if screen has changed
        if let Some(last_hash) = &self.last_hash {
            let distance = hash.dist(last_hash);
            // If very similar (distance < 5), skip saving
            if distance < 5 {
                println!("No change detected (hash distance: {})", distance);
                return Ok(());
            }
        }

        // Update last hash
        self.last_hash = Some(hash);

        // Generate timestamp-based filename
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs();

        let filename = format!("{}.webp", timestamp);
        let screenshots_dir = self.get_screenshots_dir()?;
        let filepath = screenshots_dir.join(&filename);

        // Convert to RGB8 for WebP encoding
        let rgb_img = img.to_rgb8();

        // Encode as WebP with high quality compression
        let encoder = webp::Encoder::from_rgb(&rgb_img, rgb_img.width(), rgb_img.height());
        let webp_data = encoder.encode(85.0); // 85% quality

        // Save to disk
        std::fs::write(&filepath, &*webp_data)
            .context("Failed to write WebP file")?;

        println!(
            "Captured: {} ({}x{}) -> {}",
            filename,
            img.width(),
            img.height(),
            filepath.display()
        );

        // Phase 2: OCR and Embedding Generation
        println!("Performing OCR...");
        let ocr_text = match self.extract_text_from_image(&img) {
            Ok(text) => {
                println!("OCR extracted {} characters", text.len());
                if text.len() > 100 {
                    println!("OCR text preview: {}...", &text[..100]);
                } else if !text.is_empty() {
                    println!("OCR text: {}", text);
                } else {
                    println!("OCR: No text detected");
                }
                text
            }
            Err(e) => {
                eprintln!("OCR error: {:#}", e);
                String::new()
            }
        };

        // Generate embedding
        if !ocr_text.is_empty() {
            if self.embedding_model.is_some() {
                println!("Generating embedding...");
                match self.generate_embedding(&ocr_text) {
                    Ok(embedding) => {
                        println!("✓ Embedding vector length: {}", embedding.len());
                        println!("✓ First 5 dimensions: {:?}", &embedding[..5.min(embedding.len())]);
                    }
                    Err(e) => {
                        eprintln!("Embedding generation error: {:#}", e);
                    }
                }
            } else {
                println!("⚠ Embedding model not available - skipping embedding generation");
            }
        }

        Ok(())
    }

    /// Start the capture loop (captures every 2 seconds)
    pub async fn start_capture_loop(mut self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));

        loop {
            interval.tick().await;

            if let Err(e) = self.capture_frame().await {
                eprintln!("Capture error: {:#}", e);
            }
        }
    }
}

