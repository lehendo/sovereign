import { useState } from "react";
// Auto-updater disabled - users should download manually from GitHub releases
// import { checkForUpdate, UpdateHandle, DownloadEvent } from "../lib/updaterClient";

export function UpdateBanner() {
  const [autoDismissed, setAutoDismissed] = useState(false);

  // Simulated update available (can be manually toggled for testing)
  const availableUpdate = null; // Set to { version: "1.1.5" } to test update banner

  // Commenting out auto-update check - users will check manually on GitHub
  // useEffect(() => {
  //   void handleCheckForUpdate(true);
  // }, []);

  // const handleCheckForUpdate = async (silent = false) => {
  //   ... auto-update logic disabled ...
  // };

  // const handleInstallUpdate = async () => {
  //   ... auto-install logic disabled ...
  // };

  const handleOpenGitHub = () => {
    window.open("https://github.com/lehendo/sovereign/releases/latest", "_blank");
  };

  const handleOpenLandingPage = () => {
    window.open("https://lehendo.github.io/sovereign/", "_blank");
  };

  return (
    <>
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

      {availableUpdate && !autoDismissed && (
        <div className="fixed bottom-4 left-4 z-50 w-80 rounded-2xl border border-slate-700 bg-slate-900/95 p-4 shadow-2xl backdrop-blur">
          <p className="text-sm font-semibold text-white">
            New version available: Sovereign {availableUpdate.version}
          </p>
          <p className="text-xs text-slate-400 mt-1">
            Download the latest version to get privacy and security fixes.
          </p>
          <div className="mt-3 flex gap-2">
            <button
              onClick={handleOpenGitHub}
              className="flex-1 px-3 py-2 text-sm font-semibold rounded-lg bg-indigo-600 text-white hover:bg-indigo-500"
            >
              Download
            </button>
            <button
              onClick={() => setAutoDismissed(true)}
              className="px-3 py-2 text-sm font-semibold rounded-lg border border-slate-700 text-slate-300 hover:bg-slate-800"
            >
              Dismiss
            </button>
          </div>
        </div>
      )}
    </>
  );
}

