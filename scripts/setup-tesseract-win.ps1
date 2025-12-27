# PowerShell script to download and extract portable Tesseract OCR for Windows
# This script should be run during the build process to bundle Tesseract with the app

$ErrorActionPreference = "Stop"

# Tesseract version and download URL
$TESSERACT_VERSION = "5.4.0"
$TESSERACT_URL = "https://github.com/UB-Mannheim/tesseract/releases/download/v${TESSERACT_VERSION}/tesseract-ocr-w64-setup-${TESSERACT_VERSION}.exe"
$RESOURCES_DIR = "$PSScriptRoot/../src-tauri/resources"
$TESSERACT_DIR = "$RESOURCES_DIR/tesseract-win"

Write-Host "Setting up Tesseract OCR for Windows bundling..."
Write-Host "Version: $TESSERACT_VERSION"

# Create resources directory if it doesn't exist
if (-not (Test-Path $RESOURCES_DIR)) {
    New-Item -ItemType Directory -Path $RESOURCES_DIR -Force | Out-Null
    Write-Host "Created resources directory: $RESOURCES_DIR"
}

# Remove existing Tesseract directory if it exists
if (Test-Path $TESSERACT_DIR) {
    Write-Host "Removing existing Tesseract directory..."
    Remove-Item -Path $TESSERACT_DIR -Recurse -Force
}

# Create Tesseract directory
New-Item -ItemType Directory -Path $TESSERACT_DIR -Force | Out-Null
Write-Host "Created Tesseract directory: $TESSERACT_DIR"

# Download Tesseract installer
$INSTALLER_PATH = "$env:TEMP/tesseract-installer.exe"
Write-Host "Downloading Tesseract installer from $TESSERACT_URL ..."

try {
    Invoke-WebRequest -Uri $TESSERACT_URL -OutFile $INSTALLER_PATH -UseBasicParsing
    Write-Host "Download complete: $INSTALLER_PATH"
} catch {
    Write-Error "Failed to download Tesseract installer: $_"
    exit 1
}

# Run installer silently to extract to temp location, then copy files
Write-Host "Running installer in silent mode to extract files..."
$TEMP_INSTALL_DIR = "$env:TEMP/tesseract-temp-install"
if (Test-Path $TEMP_INSTALL_DIR) {
    Remove-Item -Path $TEMP_INSTALL_DIR -Recurse -Force
}
New-Item -ItemType Directory -Path $TEMP_INSTALL_DIR -Force | Out-Null

# Run installer silently (NSIS installer)
$INSTALL_ARGS = "/S /D=$TEMP_INSTALL_DIR"
$process = Start-Process -FilePath $INSTALLER_PATH -ArgumentList $INSTALL_ARGS -Wait -NoNewWindow -PassThru

if ($process.ExitCode -ne 0) {
    Write-Error "Tesseract installer failed with exit code $($process.ExitCode)"
    exit 1
}

# Copy necessary files to our resources directory
# Tesseract typically installs to a subdirectory
$INSTALLED_TESSERACT = Get-ChildItem -Path $TEMP_INSTALL_DIR -Filter "tesseract.exe" -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1

if ($INSTALLED_TESSERACT) {
    $TESSERACT_ROOT = $INSTALLED_TESSERACT.DirectoryName
    Write-Host "Found Tesseract installation at: $TESSERACT_ROOT"
    Write-Host "Copying Tesseract files..."
    
    # Copy all files from the installation directory
    Copy-Item -Path "$TESSERACT_ROOT/*" -Destination $TESSERACT_DIR -Recurse -Force
    Write-Host "Files copied successfully!"
} else {
    Write-Error "Tesseract installation not found in expected location: $TEMP_INSTALL_DIR"
    Write-Host "Please check if the installer completed successfully."
    exit 1
}

# Clean up temp installation and installer
Remove-Item -Path $TEMP_INSTALL_DIR -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path $INSTALLER_PATH -Force -ErrorAction SilentlyContinue

# Verify installation
if (Test-Path "$TESSERACT_DIR/tesseract.exe") {
    Write-Host "✓ Tesseract setup complete!"
    Write-Host "  Location: $TESSERACT_DIR"
    Write-Host "  Executable: $TESSERACT_DIR/tesseract.exe"
    
    if (Test-Path "$TESSERACT_DIR/tessdata") {
        Write-Host "  Tessdata: $TESSERACT_DIR/tessdata"
    } else {
        Write-Warning "  Warning: tessdata directory not found"
    }
} else {
    Write-Error "Tesseract setup failed: tesseract.exe not found in $TESSERACT_DIR"
    exit 1
}

