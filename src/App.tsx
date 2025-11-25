import { useState } from "react";

function App() {
  const [isRecording] = useState(true);

  return (
    <div className="min-h-screen bg-gray-950 text-white flex items-center justify-center">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold">Sovereign</h1>
        <p className="text-gray-400">Privacy-First Screen Memory</p>
        <div className="flex items-center justify-center gap-2 text-sm">
          <div className={`w-2 h-2 rounded-full ${isRecording ? 'bg-green-500 animate-pulse' : 'bg-gray-500'}`}></div>
          <span className="text-gray-300">{isRecording ? 'Recording' : 'Paused'}</span>
        </div>
        <p className="text-xs text-gray-500 mt-8">
          Screen capture running in background
        </p>
      </div>
    </div>
  );
}

export default App;

