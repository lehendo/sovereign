import { X, Clock, FileText, Hash, ZoomIn, ZoomOut, RotateCcw } from "lucide-react";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { useState, useEffect, useRef } from "react";

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  frameId: number;
  timestamp: number;
  imagePath: string;
  ocrText?: string | null;
  similarityScore?: number;
}

export function Modal({
  isOpen,
  onClose,
  frameId,
  timestamp,
  imagePath,
  ocrText,
  similarityScore,
}: ModalProps) {
  const [imageSrc, setImageSrc] = useState<string>(convertFileSrc(imagePath));
  const [zoom, setZoom] = useState(0.25);
  const [panX, setPanX] = useState(0);
  const [panY, setPanY] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const imageContainerRef = useRef<HTMLDivElement>(null);
  const date = new Date(timestamp * 1000);

  useEffect(() => {
    if (!isOpen) {
      setZoom(0.25);
      setPanX(0);
      setPanY(0);
    }
  }, [isOpen]);

  useEffect(() => {
    const loadImage = async () => {
      const convertedSrc = convertFileSrc(imagePath);
      setImageSrc(convertedSrc);
    };
    loadImage();
  }, [imagePath]);

  const handleImageError = async (e: React.SyntheticEvent<HTMLImageElement>) => {
    const target = e.target as HTMLImageElement;
    console.error("[Modal] Failed to load image via asset protocol:", imagePath);
    try {
      const base64Data = await invoke<string>("read_image_file", { path: imagePath });
      setImageSrc(base64Data);
    } catch (error) {
      console.error("[Modal] Failed to load image via fs API:", error);
      target.style.display = 'none';
    }
  };

  const handleWheel = (e: React.WheelEvent<HTMLDivElement>) => {
    if (!e.ctrlKey && !e.metaKey) return;
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    setZoom(prev => Math.max(0.1, Math.min(5, prev * delta)));
  };

  const handleMouseDown = (e: React.MouseEvent<HTMLDivElement>) => {
    if (zoom <= 0.25) return;
    setIsDragging(true);
    setDragStart({ x: e.clientX - panX, y: e.clientY - panY });
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!isDragging || zoom <= 0.25) return;
    setPanX(e.clientX - dragStart.x);
    setPanY(e.clientY - dragStart.y);
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const resetZoom = () => {
    setZoom(0.25);
    setPanX(0);
    setPanY(0);
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 bg-black/90 backdrop-blur-sm z-50 flex items-center justify-center p-4"
      onClick={onClose}
    >
      <div
        className="w-full max-w-7xl bg-gray-900 rounded-xl border border-gray-700 shadow-2xl overflow-hidden max-h-[90vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-800">
          <div className="flex items-center gap-3">
            <Hash className="w-4 h-4 text-gray-400" />
            <span className="text-sm text-gray-400">Frame {frameId}</span>
            <div className="flex items-center gap-2 text-sm text-gray-500">
              <Clock className="w-4 h-4" />
              <span>{date.toLocaleString()}</span>
            </div>
            {similarityScore !== undefined && (
              <div className="px-2 py-1 bg-blue-500/20 text-blue-400 rounded text-xs font-mono">
                Match: {(similarityScore * 100).toFixed(1)}%
              </div>
            )}
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-800 rounded-lg transition-colors"
          >
            <X className="w-5 h-5 text-gray-400" />
          </button>
        </div>

        <div className="flex-1 overflow-auto">
          <div className="grid md:grid-cols-2 gap-4 p-4">
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <h3 className="text-sm font-semibold text-gray-300 flex items-center gap-2">
                  Screenshot
                </h3>
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setZoom(prev => Math.max(0.1, prev - 0.25))}
                    className="p-1.5 hover:bg-gray-800 rounded transition-colors"
                    title="Zoom out"
                  >
                    <ZoomOut className="w-4 h-4 text-gray-400" />
                  </button>
                  <span className="text-xs text-gray-500 min-w-[60px] text-center">
                    {Math.round(zoom * 100)}%
                  </span>
                  <button
                    onClick={() => setZoom(prev => Math.min(5, prev + 0.25))}
                    className="p-1.5 hover:bg-gray-800 rounded transition-colors"
                    title="Zoom in"
                  >
                    <ZoomIn className="w-4 h-4 text-gray-400" />
                  </button>
                  {zoom > 0.25 && (
                    <button
                      onClick={resetZoom}
                      className="p-1.5 hover:bg-gray-800 rounded transition-colors ml-2"
                      title="Reset zoom"
                    >
                      <RotateCcw className="w-4 h-4 text-gray-400" />
                    </button>
                  )}
                </div>
              </div>
              <div 
                ref={imageContainerRef}
                className="bg-gray-950 rounded-lg border border-gray-800 overflow-auto relative"
                style={{ maxHeight: '70vh' }}
                onWheel={handleWheel}
                onMouseDown={handleMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                onMouseLeave={handleMouseUp}
              >
                <div
                  style={{
                    transform: `translate(${panX}px, ${panY}px) scale(${zoom})`,
                    transformOrigin: 'top left',
                    cursor: zoom > 0.25 ? (isDragging ? 'grabbing' : 'grab') : 'default',
                  }}
                >
                  <img
                    src={imageSrc}
                    alt={`Frame ${frameId}`}
                    className="max-w-none"
                    style={{ display: 'block' }}
                    onError={handleImageError}
                    draggable={false}
                  />
                </div>
              </div>
              <p className="text-xs text-gray-500 break-all">{imagePath}</p>
              <p className="text-xs text-gray-600">
                Tip: Use Ctrl/Cmd + Mouse Wheel to zoom, drag to pan when zoomed
              </p>
            </div>

            <div className="space-y-2">
              <h3 className="text-sm font-semibold text-gray-300 flex items-center gap-2">
                <FileText className="w-4 h-4" />
                Extracted Text
              </h3>
              <div className="bg-gray-950 rounded-lg border border-gray-800 p-4 min-h-[300px] max-h-[600px] overflow-y-auto">
                {ocrText ? (
                  <pre className="text-sm text-gray-300 whitespace-pre-wrap font-mono">
                    {ocrText}
                  </pre>
                ) : (
                  <p className="text-gray-500 italic">No text detected in this frame</p>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}


