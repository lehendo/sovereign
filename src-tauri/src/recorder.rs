use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use image_hasher::{HasherConfig, ImageHash};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri::Manager;
use xcap::Monitor;

pub struct ScreenRecorder {
    last_hash: Option<ImageHash>,
    hasher: image_hasher::Hasher,
    app_handle: AppHandle,
}

impl ScreenRecorder {
    /// Create a new ScreenRecorder instance
    pub fn new(app_handle: AppHandle) -> Self {
        let hasher = HasherConfig::new()
            .hash_size(16, 16)
            .preproc_dct()
            .to_hasher();

        Self {
            last_hash: None,
            hasher,
            app_handle,
        }
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

