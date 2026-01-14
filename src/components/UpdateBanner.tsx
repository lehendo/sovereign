// Auto-updater disabled - users should download manually from GitHub releases

export function UpdateBanner() {
  const handleOpenGitHub = () => {
    window.open("https://github.com/lehendo/sovereign/releases/latest", "_blank");
  };

  const handleOpenLandingPage = () => {
    window.open("https://lehendo.github.io/sovereign/", "_blank");
  };

  return (
    <div className="bg-slate-900 border border-slate-700 rounded-2xl p-4 mb-6">
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div>
          <p className="text-sm text-slate-300 font-semibold">Updates</p>
          <p className="text-xs text-slate-500 mt-1">
            Download the latest version from GitHub or the landing page.
          </p>
        </div>
        <div className="flex gap-2">
          <button
            onClick={handleOpenGitHub}
            className="px-4 py-2 bg-indigo-600 text-white text-sm font-semibold rounded-lg hover:bg-indigo-500 transition-colors"
          >
            GitHub Releases
          </button>
          <button
            onClick={handleOpenLandingPage}
            className="px-4 py-2 bg-slate-700 text-white text-sm font-semibold rounded-lg hover:bg-slate-600 transition-colors"
          >
            Landing Page
          </button>
        </div>
      </div>
    </div>
  );
}
