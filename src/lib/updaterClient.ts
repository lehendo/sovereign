// Simplified client for the Tauri updater plugin.
// Based on @tauri-apps/plugin-updater (Apache-2.0 OR MIT).

import { invoke, Channel } from "@tauri-apps/api/core";

export type DownloadEvent =
  | { event: "Started"; data: { contentLength?: number } }
  | { event: "Progress"; data: { chunkLength: number } }
  | { event: "Finished" };

type UpdateMetadata = {
  rid: number;
  currentVersion: string;
  version: string;
  date?: string;
  body?: string;
  rawJson: Record<string, unknown>;
};

export class UpdateHandle {
  private metadata: UpdateMetadata;

  constructor(metadata: UpdateMetadata) {
    this.metadata = metadata;
  }

  get version() {
    return this.metadata.version;
  }

  get notes() {
    return this.metadata.body;
  }

  async downloadAndInstall(onEvent?: (event: DownloadEvent) => void) {
    const channel = new Channel<DownloadEvent>();
    if (onEvent) {
      channel.onmessage = onEvent;
    }

    await invoke("plugin:updater|download_and_install", {
      rid: this.metadata.rid,
      onEvent: channel,
    });
  }
}

export async function checkForUpdate(): Promise<UpdateHandle | null> {
  const metadata = await invoke<UpdateMetadata | null>("plugin:updater|check");
  return metadata ? new UpdateHandle(metadata) : null;
}

