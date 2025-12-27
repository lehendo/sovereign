#!/bin/bash
# Helper script to download GitHub Actions artifacts
# Usage: ./scripts/download-artifacts.sh [workflow-run-id]

set -e

REPO="lehendo/sovereign"
WORKFLOW_NAME="Build"

if [ -z "$1" ]; then
  echo "Usage: $0 [workflow-run-id]"
  echo ""
  echo "To get the workflow run ID:"
  echo "1. Go to https://github.com/$REPO/actions"
  echo "2. Click on a workflow run"
  echo "3. Copy the run ID from the URL (e.g., 1234567890)"
  echo ""
  echo "Or use 'latest' to get the most recent run:"
  echo "  $0 latest"
  exit 1
fi

if [ "$1" = "latest" ]; then
  echo "Fetching latest workflow run..."
  RUN_ID=$(gh run list --workflow="$WORKFLOW_NAME" --limit 1 --json databaseId --jq '.[0].databaseId')
  if [ -z "$RUN_ID" ]; then
    echo "Error: No workflow runs found"
    exit 1
  fi
  echo "Found run ID: $RUN_ID"
else
  RUN_ID="$1"
fi

echo "Downloading artifacts for run $RUN_ID..."
gh run download "$RUN_ID" --repo "$REPO"

echo ""
echo "✅ Artifacts downloaded to ./artifacts/"
echo ""
echo "To test:"
echo "  macOS: Open the .dmg file"
echo "  Windows: Install the .msi file in a VM"
echo "  Linux: Install the .deb or run the .AppImage"

