import { X, Clock, FileText, Hash } from "lucide-react";
import { convertFileSrc } from "@tauri-apps/api/core";

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
  if (!isOpen) return null;

  const imageSrc = convertFileSrc(imagePath);
  const date = new Date(timestamp * 1000);

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
              <h3 className="text-sm font-semibold text-gray-300 flex items-center gap-2">
                Screenshot
              </h3>
              <div className="bg-gray-950 rounded-lg border border-gray-800 overflow-hidden">
                <img
                  src={imageSrc}
                  alt={`Frame ${frameId}`}
                  className="w-full h-auto"
                />
              </div>
              <p className="text-xs text-gray-500 break-all">{imagePath}</p>
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


