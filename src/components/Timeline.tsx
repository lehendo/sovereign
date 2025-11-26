import { Clock, Calendar } from "lucide-react";
import { useState } from "react";

interface TimelineProps {
  totalFrames: number;
  onRangeChange: (startTime: number, endTime: number) => void;
}

export function Timeline({ totalFrames, onRangeChange }: TimelineProps) {
  const [range, setRange] = useState({ start: 0, end: 100 });

  const handleStartChange = (value: number) => {
    setRange((prev) => ({ ...prev, start: value }));
    onRangeChange(value, range.end);
  };

  const handleEndChange = (value: number) => {
    setRange((prev) => ({ ...prev, end: value }));
    onRangeChange(range.start, value);
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
        {/* Time range slider */}
        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs text-gray-500">
            <span>Start</span>
            <span>{range.start}%</span>
          </div>
          <input
            type="range"
            min="0"
            max="100"
            value={range.start}
            onChange={(e) => handleStartChange(Number(e.target.value))}
            className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
          />
        </div>

        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs text-gray-500">
            <span>End</span>
            <span>{range.end}%</span>
          </div>
          <input
            type="range"
            min="0"
            max="100"
            value={range.end}
            onChange={(e) => handleEndChange(Number(e.target.value))}
            className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
          />
        </div>

        <div className="flex items-center gap-2 pt-2 border-t border-gray-800">
          <Calendar className="w-4 h-4 text-gray-500" />
          <span className="text-xs text-gray-400">
            Showing {range.start}% - {range.end}% of history
          </span>
        </div>
      </div>
    </div>
  );
}


