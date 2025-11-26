import { useState } from "react";
import { QueryClient, QueryClientProvider, useQuery, useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { SearchBar } from "./components/SearchBar";
import { Timeline } from "./components/Timeline";
import { Grid } from "./components/Grid";
import { Modal } from "./components/Modal";
import { Activity, Database as DatabaseIcon, Shield } from "lucide-react";
import type { FrameMetadata, DatabaseStats } from "./types";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});

function AppContent() {
  const [selectedFrame, setSelectedFrame] = useState<FrameMetadata | null>(null);
  const [searchResults, setSearchResults] = useState<FrameMetadata[]>([]);
  const [isSearchMode, setIsSearchMode] = useState(false);

  // Fetch recent frames on load
  const { data: recentFrames = [], isLoading: isLoadingRecent } = useQuery<FrameMetadata[]>({
    queryKey: ["recentFrames"],
    queryFn: async () => {
      return await invoke<FrameMetadata[]>("get_recent_frames", { limit: 50 });
    },
    refetchInterval: 5000, // Refresh every 5 seconds
  });

  // Fetch database stats
  const { data: stats } = useQuery<DatabaseStats>({
    queryKey: ["databaseStats"],
    queryFn: async () => {
      return await invoke<DatabaseStats>("get_database_stats");
    },
    refetchInterval: 10000,
  });

  // Search mutation
  const searchMutation = useMutation({
    mutationFn: async (query: string) => {
      return await invoke<FrameMetadata[]>("search_frames", { query });
    },
    onSuccess: (data) => {
      setSearchResults(data);
      setIsSearchMode(true);
    },
  });

  const handleSearch = (query: string) => {
    searchMutation.mutate(query);
  };

  const handleTimelineChange = (start: number, end: number) => {
    console.log("Timeline range:", start, end);
    // TODO: Filter frames by time range
  };

  const displayedFrames = isSearchMode ? searchResults : recentFrames;

  return (
    <div className="min-h-screen bg-gray-950 text-white">
      {/* Header */}
      <header className="border-b border-gray-800 bg-gray-900/50 backdrop-blur-sm sticky top-0 z-40">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Activity className="w-6 h-6 text-blue-500" />
              <h1 className="text-2xl font-bold">Sovereign</h1>
              <span className="text-xs text-gray-500 px-2 py-1 bg-gray-800 rounded">
                Privacy-First Screen Memory
              </span>
            </div>

            {stats && (
              <div className="flex items-center gap-4 text-sm">
                <div className="flex items-center gap-2 px-3 py-1 bg-green-500/10 border border-green-500/30 rounded-lg">
                  <Shield className="w-4 h-4 text-green-500" />
                  <span className="text-green-400">Privacy Guard Active</span>
                </div>
                <div className="flex items-center gap-2 px-3 py-1 bg-gray-800 rounded-lg">
                  <DatabaseIcon className="w-4 h-4 text-gray-400" />
                  <span className="text-gray-400">{stats.total_frames} frames</span>
                </div>
                {isLoadingRecent && (
                  <div className="flex items-center gap-2 text-gray-500">
                    <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse" />
                    <span className="text-xs">Recording...</span>
                  </div>
                )}
              </div>
            )}
          </div>

          <div className="flex items-center gap-4">
            <div className="flex-1">
              <SearchBar
                onSearch={handleSearch}
                isLoading={searchMutation.isPending}
              />
            </div>
            {isSearchMode && (
              <button
                onClick={() => {
                  setIsSearchMode(false);
                  setSearchResults([]);
                }}
                className="px-4 py-2 text-sm bg-gray-800 hover:bg-gray-700 rounded-lg border border-gray-700 transition-colors"
              >
                Clear Search
              </button>
            )}
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-6">
        <div className="grid lg:grid-cols-4 gap-6">
          {/* Sidebar */}
          <aside className="lg:col-span-1 space-y-4">
            <Timeline
              totalFrames={stats?.total_frames || 0}
              onRangeChange={handleTimelineChange}
            />

            {/* Privacy Guard Card */}
            <div className="bg-gray-900 border border-gray-800 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-3">
                <Shield className="w-4 h-4 text-green-500" />
                <h3 className="text-sm font-semibold text-gray-300">Privacy Guard</h3>
              </div>
              <p className="text-xs text-gray-500 leading-relaxed">
                Automatically skips recording when sensitive windows are detected (password managers, private browsing, etc.)
              </p>
              <div className="mt-3 px-2 py-1 bg-green-500/10 border border-green-500/30 rounded text-xs text-green-400 text-center">
                Active
              </div>
            </div>

            {/* Stats Card */}
            {stats && (
              <div className="bg-gray-900 border border-gray-800 rounded-lg p-4">
                <h3 className="text-sm font-semibold text-gray-300 mb-3">Statistics</h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-gray-500">Frames</span>
                    <span className="text-white font-mono">{stats.total_frames}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">OCR Entries</span>
                    <span className="text-white font-mono">{stats.total_ocr_entries}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500">Embeddings</span>
                    <span className="text-white font-mono">{stats.total_embeddings}</span>
                  </div>
                  <div className="flex justify-between pt-2 border-t border-gray-800">
                    <span className="text-gray-500">Retention</span>
                    <span className="text-white font-mono">14 days</span>
                  </div>
                </div>
              </div>
            )}
          </aside>

          {/* Grid */}
          <div className="lg:col-span-3">
            {isSearchMode && (
              <div className="mb-4 p-3 bg-blue-500/10 border border-blue-500/30 rounded-lg">
                <p className="text-sm text-blue-400">
                  Found {searchResults.length} results
                </p>
              </div>
            )}

            {searchMutation.isPending ? (
              <div className="flex items-center justify-center py-16">
                <div className="text-center">
                  <div className="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin mx-auto mb-4" />
                  <p className="text-gray-400">Searching...</p>
                </div>
              </div>
            ) : (
              <Grid items={displayedFrames} onItemClick={setSelectedFrame} />
            )}
          </div>
        </div>
      </main>

      {/* Modal */}
      {selectedFrame && (
        <Modal
          isOpen={!!selectedFrame}
          onClose={() => setSelectedFrame(null)}
          frameId={selectedFrame.frame_id}
          timestamp={selectedFrame.timestamp}
          imagePath={selectedFrame.image_path}
          ocrText={selectedFrame.ocr_text}
          similarityScore={selectedFrame.similarity_score}
        />
      )}
    </div>
  );
}

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AppContent />
    </QueryClientProvider>
  );
}
