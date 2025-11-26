use anyhow::Result;
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::Database;
use crate::search::{cosine_similarity, FrameMetadata, SearchResult};
use fastembed::TextEmbedding;

/// Shared application state
pub struct AppState {
    pub database: Arc<Mutex<Database>>,
    pub embedding_model: Arc<Mutex<Option<TextEmbedding>>>,
}

/// Search frames using natural language query
#[tauri::command]
pub async fn search_frames(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    println!("Search query: '{}'", query);

    if query.trim().is_empty() {
        return Ok(vec![]);
    }

    // Validate and sanitize query input
    let query = query.trim();
    if query.is_empty() {
        return Ok(vec![]);
    }
    
    // Limit query length to prevent DoS
    if query.len() > 1000 {
        return Err("Query too long. Maximum 1000 characters.".to_string());
    }

    // Generate query embedding
    let embedding_model = state.embedding_model.lock()
        .map_err(|_| "Database lock error".to_string())?;
    let query_vector = match embedding_model.as_ref() {
        Some(model) => {
            let embeddings = model
                .embed(vec![query], None)
                .map_err(|e| format!("Failed to generate query embedding: {}", e))?;
            
            embeddings.into_iter().next().unwrap_or_default()
        }
        None => {
            return Err("Embedding model not available. Please download the model files.".to_string());
        }
    };
    drop(embedding_model);

    println!("Query embedding generated ({} dimensions)", query_vector.len());

    // Fetch all embeddings from database
    let db = state.database.lock()
        .map_err(|_| "Database lock error".to_string())?;
    let all_embeddings = db
        .get_all_embeddings()
        .map_err(|e| format!("Failed to fetch embeddings: {}", e))?;

    println!("Comparing against {} stored embeddings", all_embeddings.len());

    if all_embeddings.is_empty() {
        return Ok(vec![]);
    }

    // Calculate similarity scores for all embeddings
    let mut scores: Vec<(i64, f32)> = all_embeddings
        .iter()
        .map(|(frame_id, vector)| {
            let score = cosine_similarity(&query_vector, vector);
            (*frame_id, score)
        })
        .collect();

    // Sort by score descending
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Take top 20 results
    let top_results: Vec<(i64, f32)> = scores.into_iter().take(20).collect();

    println!("Top {} results found", top_results.len());

    // Fetch metadata for top results
    let frame_ids: Vec<i64> = top_results.iter().map(|(id, _)| *id).collect();
    let frames = db
        .get_frames_by_ids(&frame_ids)
        .map_err(|e| format!("Failed to fetch frame metadata: {}", e))?;

    drop(db);

    // Build search results with scores
    let mut results: Vec<SearchResult> = frames
        .into_iter()
        .filter_map(|frame| {
            // Find the score for this frame
            let score = top_results
                .iter()
                .find(|(id, _)| *id == frame.frame_id)
                .map(|(_, score)| *score)
                .unwrap_or(0.0);

            Some(SearchResult {
                frame_id: frame.frame_id,
                timestamp: frame.timestamp,
                image_path: frame.image_path,
                ocr_text: frame.ocr_text.unwrap_or_default(),
                similarity_score: score,
            })
        })
        .collect();

    // Sort by score again (in case DB didn't return in order)
    results.sort_by(|a, b| {
        b.similarity_score
            .partial_cmp(&a.similarity_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    println!(
        "Returning {} search results (top score: {:.4})",
        results.len(),
        results.first().map(|r| r.similarity_score).unwrap_or(0.0)
    );

    Ok(results)
}

/// Get the most recent N frames for timeline view
#[tauri::command]
pub async fn get_recent_frames(
    limit: usize,
    state: State<'_, AppState>,
) -> Result<Vec<FrameMetadata>, String> {
    // Validate limit to prevent DoS
    let limit = limit.min(1000);
    
    let db = state.database.lock()
        .map_err(|_| "Database lock error".to_string())?;
    
    let frames = db
        .get_recent_frames(limit)
        .map_err(|e| format!("Failed to fetch recent frames: {}", e))?;

    println!("Retrieved {} recent frames", frames.len());

    Ok(frames)
}

/// Get database statistics
#[tauri::command]
pub async fn get_database_stats(
    state: State<'_, AppState>,
) -> Result<crate::database::DatabaseStats, String> {
    let db = state.database.lock()
        .map_err(|_| "Database lock error".to_string())?;
    
    let stats = db
        .get_stats()
        .map_err(|e| format!("Failed to get stats: {}", e))?;

    Ok(stats)
}


