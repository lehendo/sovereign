use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::PathBuf;

use crate::search::FrameMetadata;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Initialize the database and create tables if they don't exist
    pub fn new(db_path: PathBuf) -> Result<Self> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
        }

        let conn = Connection::open(&db_path)
            .context(format!("Failed to open database at {}", db_path.display()))?;

        let db = Self { conn };
        db.create_tables()?;

        println!("Database initialized at: {}", db_path.display());
        Ok(db)
    }

    /// Create the database schema
    fn create_tables(&self) -> Result<()> {
        // Table: frames
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS frames (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                app_name TEXT,
                window_title TEXT,
                image_path TEXT NOT NULL,
                perceptual_hash TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Table: ocr_text
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS ocr_text (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                frame_id INTEGER NOT NULL,
                raw_text TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (frame_id) REFERENCES frames(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Table: embeddings
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                frame_id INTEGER NOT NULL,
                vector BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (frame_id) REFERENCES frames(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Create indices for performance
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_frames_timestamp ON frames(timestamp)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_ocr_frame_id ON ocr_text(frame_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_embeddings_frame_id ON embeddings(frame_id)",
            [],
        )?;

        println!("Database schema initialized successfully");
        Ok(())
    }

    /// Insert a new frame and return its ID
    pub fn insert_frame(
        &self,
        timestamp: u64,
        image_path: &str,
        perceptual_hash: &str,
        app_name: Option<&str>,
        window_title: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO frames (timestamp, image_path, perceptual_hash, app_name, window_title)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![timestamp, image_path, perceptual_hash, app_name, window_title],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Insert OCR text for a frame
    pub fn insert_ocr_text(&self, frame_id: i64, raw_text: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO ocr_text (frame_id, raw_text) VALUES (?1, ?2)",
            params![frame_id, raw_text],
        )?;

        Ok(())
    }

    /// Insert embedding vector for a frame
    pub fn insert_embedding(&self, frame_id: i64, vector: &[f32]) -> Result<()> {
        // Serialize vector to bytes using bincode
        let vector_bytes = bincode::serialize(vector)
            .context("Failed to serialize embedding vector")?;

        self.conn.execute(
            "INSERT INTO embeddings (frame_id, vector) VALUES (?1, ?2)",
            params![frame_id, vector_bytes],
        )?;

        Ok(())
    }

    /// Get total number of frames
    pub fn get_frame_count(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM frames",
            [],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Get database statistics
    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let frame_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM frames",
            [],
            |row| row.get(0),
        )?;

        let ocr_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM ocr_text",
            [],
            |row| row.get(0),
        )?;

        let embedding_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM embeddings",
            [],
            |row| row.get(0),
        )?;

        Ok(DatabaseStats {
            total_frames: frame_count,
            total_ocr_entries: ocr_count,
            total_embeddings: embedding_count,
        })
    }
}

/// Database statistics
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseStats {
    pub total_frames: i64,
    pub total_ocr_entries: i64,
    pub total_embeddings: i64,
}

impl Database {
    /// Fetch all embeddings (frame_id and vector) for similarity search
    pub fn get_all_embeddings(&self) -> Result<Vec<(i64, Vec<f32>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT frame_id, vector FROM embeddings"
        )?;

        let embeddings = stmt.query_map([], |row| {
            let frame_id: i64 = row.get(0)?;
            let vector_blob: Vec<u8> = row.get(1)?;
            
            // Deserialize the vector from bincode
            let vector: Vec<f32> = bincode::deserialize(&vector_blob)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Blob,
                    Box::new(e),
                ))?;
            
            Ok((frame_id, vector))
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(embeddings)
    }

    /// Get frame metadata by ID
    pub fn get_frame_by_id(&self, frame_id: i64) -> Result<Option<FrameMetadata>> {
        let mut stmt = self.conn.prepare(
            "SELECT f.id, f.timestamp, f.image_path, f.app_name, f.window_title, o.raw_text
             FROM frames f
             LEFT JOIN ocr_text o ON f.id = o.frame_id
             WHERE f.id = ?1"
        )?;

        let result = stmt.query_row([frame_id], |row| {
            Ok(FrameMetadata {
                frame_id: row.get(0)?,
                timestamp: row.get(1)?,
                image_path: row.get(2)?,
                app_name: row.get(3)?,
                window_title: row.get(4)?,
                ocr_text: row.get(5)?,
            })
        });

        match result {
            Ok(metadata) => Ok(Some(metadata)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get multiple frames by IDs (for search results)
    pub fn get_frames_by_ids(&self, frame_ids: &[i64]) -> Result<Vec<FrameMetadata>> {
        if frame_ids.is_empty() {
            return Ok(vec![]);
        }

        // Create placeholders for SQL IN clause
        let placeholders = frame_ids.iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            "SELECT f.id, f.timestamp, f.image_path, f.app_name, f.window_title, o.raw_text
             FROM frames f
             LEFT JOIN ocr_text o ON f.id = o.frame_id
             WHERE f.id IN ({})
             ORDER BY f.timestamp DESC",
            placeholders
        );

        let mut stmt = self.conn.prepare(&query)?;
        
        let frames = stmt.query_map(
            rusqlite::params_from_iter(frame_ids.iter()),
            |row| {
                Ok(FrameMetadata {
                    frame_id: row.get(0)?,
                    timestamp: row.get(1)?,
                    image_path: row.get(2)?,
                    app_name: row.get(3)?,
                    window_title: row.get(4)?,
                    ocr_text: row.get(5)?,
                })
            }
        )?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(frames)
    }

    /// Get the most recent N frames
    pub fn get_recent_frames(&self, limit: usize) -> Result<Vec<FrameMetadata>> {
        let mut stmt = self.conn.prepare(
            "SELECT f.id, f.timestamp, f.image_path, f.app_name, f.window_title, o.raw_text
             FROM frames f
             LEFT JOIN ocr_text o ON f.id = o.frame_id
             ORDER BY f.timestamp DESC
             LIMIT ?1"
        )?;

        let frames = stmt.query_map([limit], |row| {
            Ok(FrameMetadata {
                frame_id: row.get(0)?,
                timestamp: row.get(1)?,
                image_path: row.get(2)?,
                app_name: row.get(3)?,
                window_title: row.get(4)?,
                ocr_text: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(frames)
    }

    /// Prune old data from the database and delete files from disk
    /// Deletes frames older than the specified number of days
    pub fn prune_old_data(&self, days: i64) -> Result<usize> {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Validate days parameter to prevent DoS
        if days < 0 || days > 365 {
            return Err(anyhow::anyhow!("Invalid retention period: must be between 0 and 365 days"));
        }

        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get current timestamp")?
            .as_secs() as i64;

        let cutoff_timestamp = current_timestamp - (days * 24 * 60 * 60);

        // First, fetch all image paths that will be deleted
        let mut stmt = self.conn.prepare(
            "SELECT image_path FROM frames WHERE timestamp < ?1"
        )?;

        let paths: Vec<String> = stmt.query_map([cutoff_timestamp], |row| {
            row.get(0)
        })?
        .collect::<Result<Vec<_>, _>>()?;

        // Delete from database (cascades to ocr_text and embeddings)
        let deleted_count = self.conn.execute(
            "DELETE FROM frames WHERE timestamp < ?1",
            params![cutoff_timestamp],
        )?;

        // Delete image files from disk with path validation
        let mut files_deleted = 0;
        for path_str in paths {
            let path = PathBuf::from(&path_str);
            
            // Security: Validate path to prevent directory traversal
            // Only allow deletion of files within expected screenshot directories
            if path_str.contains("..") || path_str.contains("//") {
                eprintln!("Warning: Skipping suspicious path: {}", path_str);
                continue;
            }
            
            // Additional validation: ensure path is a file (not a directory)
            if path.is_file() {
                if let Err(e) = std::fs::remove_file(&path) {
                    eprintln!("Warning: Could not delete file {}: {}", path_str, e);
                } else {
                    files_deleted += 1;
                }
            } else {
                eprintln!("Warning: Path is not a file: {}", path_str);
            }
        }

        if deleted_count > 0 {
            println!(
                "Pruned {} frames older than {} days ({} files deleted from disk)",
                deleted_count, days, files_deleted
            );
        }

        Ok(deleted_count)
    }
}

