import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { Image as ImageIcon, FileText } from "lucide-react";
import { useState, useEffect, useCallback } from "react";

interface GridItem {
  frame_id: number;
  timestamp: number;
  image_path: string;
  ocr_text?: string | null;
  similarity_score?: number;
}

interface GridProps {
  items: GridItem[];
  onItemClick: (item: GridItem) => void;
}

export function Grid({ items, onItemClick }: GridProps) {
  if (items.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-gray-500">
        <ImageIcon className="w-12 h-12 mb-4 opacity-50" />
        <p className="text-lg">No frames to display</p>
        <p className="text-sm mt-2">Frames will appear as they are captured</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      {items.map((item) => (
        <GridItem key={item.frame_id} item={item} onItemClick={onItemClick} />
      ))}
    </div>
  );
}

function GridItem({ item, onItemClick }: { item: GridItem; onItemClick: (item: GridItem) => void }) {
  const [imageSrc, setImageSrc] = useState<string>(convertFileSrc(item.image_path));
  const date = new Date(item.timestamp * 1000);

  useEffect(() => {
    const loadImage = async () => {
      try {
        const convertedSrc = convertFileSrc(item.image_path);
        setImageSrc(convertedSrc);
      } catch (error) {
        console.error("[Grid] Failed to convert file src:", error);
      }
    };
    loadImage();
  }, [item.image_path]);

  const handleImageError = useCallback(async (e: React.SyntheticEvent<HTMLImageElement>) => {
    const target = e.target as HTMLImageElement;
    console.error("[Grid] Failed to load image via asset protocol:", item.image_path);
    try {
      const base64Data = await invoke<string>("read_image_file", { path: item.image_path });
      setImageSrc(base64Data);
    } catch (error) {
      console.error("[Grid] Failed to load image via fs API:", error);
      target.style.display = 'none';
    }
  }, [item.image_path]);

  return (
    <div
      onClick={() => onItemClick(item)}
      className="group relative bg-gray-800 rounded-lg overflow-hidden border border-gray-700 hover:border-blue-500 cursor-pointer transition-all hover:scale-105"
    >
      <div className="aspect-video bg-gray-900 relative overflow-hidden">
        <img
          src={imageSrc}
          alt={`Frame ${item.frame_id}`}
          className="w-full h-full object-contain"
          loading="lazy"
          onError={handleImageError}
        />
        
        <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
          <ImageIcon className="w-8 h-8 text-white" />
        </div>
      </div>

      <div className="p-3 space-y-1">
        <div className="flex items-center justify-between text-xs text-gray-400">
          <span>{date.toLocaleString(undefined, {
            month: "short",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
          })}</span>
          {item.similarity_score !== undefined && (
            <span className="text-blue-400 font-mono">
              {(item.similarity_score * 100).toFixed(1)}%
            </span>
          )}
        </div>
        
        {item.ocr_text && (
          <div className="flex items-start gap-1 text-xs text-gray-500">
            <FileText className="w-3 h-3 mt-0.5 flex-shrink-0" />
            <p className="line-clamp-2">{item.ocr_text}</p>
          </div>
        )}
      </div>
    </div>
  );
}


