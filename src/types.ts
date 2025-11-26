export interface FrameMetadata {
  frame_id: number;
  timestamp: number;
  image_path: string;
  app_name?: string | null;
  window_title?: string | null;
  ocr_text?: string | null;
  similarity_score?: number;
}

export interface DatabaseStats {
  total_frames: number;
  total_ocr_entries: number;
  total_embeddings: number;
  oldest_timestamp?: number;
  newest_timestamp?: number;
}

