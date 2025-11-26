use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::PathBuf;

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
#[derive(Debug)]
pub struct DatabaseStats {
    pub total_frames: i64,
    pub total_ocr_entries: i64,
    pub total_embeddings: i64,
}

