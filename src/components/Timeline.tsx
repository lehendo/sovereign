import { Clock, Calendar } from "lucide-react";

interface TimelineProps {
  totalFrames: number;
  oldestTimestamp?: number | null;
  newestTimestamp?: number | null;
  range: { start: number; end: number } | null;
  onRangeChange: (startTime: number, endTime: number) => void;
}

const MIN_WINDOW_SECONDS = 60;

export function Timeline({
  totalFrames,
  oldestTimestamp,
  newestTimestamp,
  range,
  onRangeChange,
}: TimelineProps) {
  const hasFrames =
    typeof oldestTimestamp === "number" &&
    typeof newestTimestamp === "number" &&
    totalFrames > 0;

  if (!hasFrames) {
    return (
      <div className="bg-gray-900 border border-gray-800 rounded-lg p-4">
        <div className="flex items-center gap-2 mb-2">
          <Clock className="w-4 h-4 text-gray-400" />
          <h3 className="text-sm font-semibold text-gray-300">Timeline</h3>
        </div>
        <p className="text-xs text-gray-500">No frames captured yet.</p>
      </div>
    );
  }

  const min = oldestTimestamp as number;
  const max = newestTimestamp as number;
  const startValue = range?.start ?? min;
  const endValue = range?.end ?? max;

  const formatTimestamp = (value: number) =>
    new Date(value * 1000).toLocaleString(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });

  const clampStart = (value: number) => {
    const clamped = Math.max(min, Math.min(value, max - MIN_WINDOW_SECONDS));
    return Math.min(clamped, endValue - MIN_WINDOW_SECONDS);
  };

  const clampEnd = (value: number) => {
    const clamped = Math.min(max, Math.max(value, min + MIN_WINDOW_SECONDS));
    return Math.max(clamped, startValue + MIN_WINDOW_SECONDS);
  };

  const handleStartChange = (value: number) => {
    const newStart = clampStart(value);
    onRangeChange(newStart, endValue);
  };

  const handleEndChange = (value: number) => {
    const newEnd = clampEnd(value);
    onRangeChange(startValue, newEnd);
  };

  return (
    <div className="bg-gray-900 border border-gray-800 rounded-lg p-4">
      <div className="flex items-center gap-2 mb-4">
        <Clock className="w-4 h-4 text-gray-400" />
        <h3 className="text-sm font-semibold text-gray-300">Timeline</h3>
        <span className="text-xs text-gray-500 ml-auto">
          {totalFrames} frames captured
        </span>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs text-gray-500">
            <span>Start</span>
            <span>{formatTimestamp(startValue)}</span>
          </div>
          <input
            type="range"
            min={min}
            max={max - MIN_WINDOW_SECONDS}
            value={startValue}
            onChange={(e) => handleStartChange(Number(e.target.value))}
            className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
          />
        </div>

        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs text-gray-500">
            <span>End</span>
            <span>{formatTimestamp(endValue)}</span>
          </div>
          <input
            type="range"
            min={min + MIN_WINDOW_SECONDS}
            max={max}
            value={endValue}
            onChange={(e) => handleEndChange(Number(e.target.value))}
            className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
          />
        </div>

        <div className="flex items-center gap-2 pt-2 border-t border-gray-800">
          <Calendar className="w-4 h-4 text-gray-500" />
          <span className="text-xs text-gray-400 leading-tight">
            Showing frames captured between{" "}
            <span className="text-white">{formatTimestamp(startValue)}</span> and{" "}
            <span className="text-white">{formatTimestamp(endValue)}</span>
          </span>
        </div>
      </div>
    </div>
  );
}
