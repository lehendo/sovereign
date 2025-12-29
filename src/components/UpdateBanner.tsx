import { useEffect, useState } from "react";
import { checkForUpdate, UpdateHandle, DownloadEvent } from "../lib/updaterClient";

type UpdateStatus = "idle" | "checking" | "upToDate" | "available" | "installing" | "error";

export function UpdateBanner() {
  const [status, setStatus] = useState<UpdateStatus>("idle");
  const [message, setMessage] = useState<string>("");
  const [availableUpdate, setAvailableUpdate] = useState<UpdateHandle | null>(null);
  const [autoDismissed, setAutoDismissed] = useState(false);

  useEffect(() => {
    void handleCheckForUpdate(true);
  }, []);

  const handleCheckForUpdate = async (silent = false) => {
    if (!silent) {
      setStatus("checking");
      setMessage("Checking for updates…");
    }
    setAvailableUpdate(null);

    try {
      const update = await checkForUpdate();

      if (!update) {
        if (!silent) {
          setStatus("upToDate");
          setMessage("You are running the latest version.");
        }
        return;
      }

      setAvailableUpdate(update);
      setAutoDismissed(false);
      setStatus("available");
      setMessage(`Sovereign ${update.version} is ready to install.`);
    } catch (error) {
      console.error("[Updater] Failed to check updates", error);
      if (!silent) {
        const errorStr = error instanceof Error ? error.message : String(error || "");
        const errorFull = error instanceof Error ? error.toString() : String(error || "");
        console.error("[Updater] Error details:", errorStr);
        console.error("[Updater] Full error:", errorFull);
        if (errorStr.includes("404") || errorStr.includes("Not Found") || errorStr.includes("latest.json")) {
          setStatus("error");
          setMessage(`Update check failed: latest.json not found. Error: ${errorStr.substring(0, 100)}`);
        } else if (errorStr.includes("network") || errorStr.includes("fetch") || errorStr.includes("Failed to fetch")) {
          setStatus("error");
          setMessage("Network error: Unable to reach update server. Check your internet connection.");
        } else if (errorStr.includes("signature") || errorStr.includes("pubkey") || errorStr.includes("key") || errorStr.includes("verification") || errorStr.includes("signing")) {
          setStatus("error");
          setMessage(`Signature verification failed. Your app version may be too old. Please download v1.1.2 manually from GitHub. Error: ${errorStr.substring(0, 120)}`);
        } else {
          setStatus("error");
          setMessage(`Unable to check for updates: ${errorStr.substring(0, 150)}`);
        }
      }
    }
  };

  const handleInstallUpdate = async () => {
    if (!availableUpdate) return;

    try {
      setStatus("installing");
      setMessage("Installing update… Sovereign will restart.");
      await availableUpdate.downloadAndInstall(handleDownloadEvent);
    } catch (error) {
      console.error("[Updater] Failed to install update", error);
      const errorStr = error instanceof Error ? error.message : String(error || "");
      const errorFull = error instanceof Error ? error.toString() : String(error || "");
      console.error("[Updater] Install error details:", errorStr);
      console.error("[Updater] Full install error:", errorFull);
      
      setStatus("error");
      
      // Provide more specific error messages
      if (errorStr.includes("permission") || errorStr.includes("denied") || errorStr.includes("access")) {
        setMessage(`Update installation failed: Permission denied. Please download v${availableUpdate.version} manually from GitHub.`);
      } else if (errorStr.includes("network") || errorStr.includes("fetch") || errorStr.includes("download")) {
        setMessage(`Update download failed: Network error. Please check your internet connection and try again, or download manually.`);
      } else if (errorStr.includes("signature") || errorStr.includes("verify")) {
        setMessage(`Update verification failed: Signature error. Please download v${availableUpdate.version} manually from GitHub.`);
      } else {
        setMessage(`Auto-update failed: ${errorStr.substring(0, 100)}. Please download v${availableUpdate.version} manually from GitHub.`);
      }
    }
  };

  const handleDownloadEvent = (event: DownloadEvent) => {
    if (event.event === "Progress") {
      setMessage(`Downloading update…`);
    }
  };

  const renderStatus = () => {
    switch (status) {
      case "checking":
        return message;
      case "upToDate":
        return message;
      case "available":
        return message;
      case "installing":
        return message;
      case "error":
        return message;
      default:
        return "Stay secure with the latest privacy and security patches.";
    }
  };

  return (
    <>
      <div className="bg-slate-900 border border-slate-700 rounded-2xl p-4 mb-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
          <div>
            <p className="text-sm text-slate-300 font-semibold">Auto Updates</p>
            <p className="text-xs text-slate-500 mt-1">{renderStatus()}</p>
          </div>
          <button
            onClick={() => handleCheckForUpdate(false)}
            className="px-4 py-2 bg-indigo-600 text-white text-sm font-semibold rounded-lg hover:bg-indigo-500 transition-colors disabled:opacity-60"
            disabled={status === "checking" || status === "installing"}
          >
            {status === "checking" ? "Checking…" : status === "installing" ? "Installing…" : "Check for Updates"}
          </button>
        {status === "available" && (
            <button
              onClick={handleInstallUpdate}
              className="px-4 py-2 bg-green-600 text-white text-sm font-semibold rounded-lg hover:bg-green-500 transition-colors"
            >
            Install {availableUpdate?.version}
            </button>
          )}
        </div>
      </div>

      {availableUpdate && !autoDismissed && (
        <div className="fixed bottom-4 left-4 z-50 w-80 rounded-2xl border border-slate-700 bg-slate-900/95 p-4 shadow-2xl backdrop-blur">
          <p className="text-sm font-semibold text-white">
            Sovereign {availableUpdate.version} is ready
          </p>
          <p className="text-xs text-slate-400 mt-1">
            Install now to get the latest privacy and security fixes.
          </p>
          <div className="mt-3 flex gap-2">
            <button
              onClick={handleInstallUpdate}
              className="flex-1 px-3 py-2 text-sm font-semibold rounded-lg bg-green-600 text-white hover:bg-green-500"
            >
              Update
            </button>
            <button
              onClick={() => setAutoDismissed(true)}
              className="px-3 py-2 text-sm font-semibold rounded-lg border border-slate-700 text-slate-300 hover:bg-slate-800"
            >
              Not right now
            </button>
          </div>
        </div>
      )}
    </>
  );
}

