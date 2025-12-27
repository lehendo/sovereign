# PowerShell script to download and extract portable Tesseract OCR for Windows
# This script should be run during the build process to bundle Tesseract with the app

$ErrorActionPreference = "Stop"

# Tesseract version - try multiple versions in order
$TESSERACT_VERSIONS = @("5.4.1", "5.4.0", "5.3.3", "5.3.2")
$RESOURCES_DIR = "$PSScriptRoot/../src-tauri/resources"
$TESSERACT_DIR = "$RESOURCES_DIR/tesseract-win"

Write-Host "Setting up Tesseract OCR for Windows bundling..."

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

# Try installation methods in order of reliability for CI environments
$DOWNLOAD_SUCCESS = $false
$TESSERACT_VERSION = $null
$INSTALLER_PATH = "$env:TEMP/tesseract-installer.exe"

# Method 1: Try using winget (Windows Package Manager) - most reliable in CI
Write-Host "Attempting to install Tesseract via winget (recommended for CI)..."
try {
    $wingetCheck = winget --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Winget detected. Installing Tesseract OCR..."
        winget install --id UB-Mannheim.TesseractOCR --silent --accept-package-agreements --accept-source-agreements 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            # Find the installed Tesseract location (usually in Program Files)
            $possiblePaths = @(
                "${env:ProgramFiles}\Tesseract-OCR",
                "${env:ProgramFiles(x86)}\Tesseract-OCR"
            )
            foreach ($path in $possiblePaths) {
                if (Test-Path "$path\tesseract.exe") {
                    Write-Host "Found Tesseract installation at: $path"
                    Write-Host "Copying Tesseract files to bundle directory..."
                    Copy-Item -Path "$path\*" -Destination $TESSERACT_DIR -Recurse -Force
                    $DOWNLOAD_SUCCESS = $true
                    Write-Host "✓ Tesseract installed and bundled via winget"
                    break
                }
            }
        } else {
            Write-Host "Winget installation returned non-zero exit code"
        }
    }
} catch {
    Write-Host "Winget not available: $_"
}

# Method 2: Try using Chocolatey if winget failed
if (-not $DOWNLOAD_SUCCESS) {
    Write-Host "Attempting to install Tesseract via Chocolatey..."
    try {
        $chocoCheck = choco --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Chocolatey detected. Installing Tesseract OCR..."
            choco install tesseract -y --no-progress 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                # Find the installed Tesseract location
                $tesseractPath = (Get-Command tesseract -ErrorAction SilentlyContinue).Source
                if ($tesseractPath) {
                    $tesseractInstallDir = Split-Path $tesseractPath -Parent
                    Write-Host "Found Tesseract installation at: $tesseractInstallDir"
                    Write-Host "Copying Tesseract files to bundle directory..."
                    Copy-Item -Path "$tesseractInstallDir\*" -Destination $TESSERACT_DIR -Recurse -Force
                    $DOWNLOAD_SUCCESS = $true
                    Write-Host "✓ Tesseract installed and bundled via Chocolatey"
                }
            }
        }
    } catch {
        Write-Host "Chocolatey not available: $_"
    }
}

# Method 3: Try direct download as last resort (URLs may be unreliable)
if (-not $DOWNLOAD_SUCCESS) {
    Write-Host "Package managers not available. Attempting direct download (may be unreliable)..."
    foreach ($version in $TESSERACT_VERSIONS) {
        $TESSERACT_URL = "https://github.com/UB-Mannheim/tesseract/releases/download/v${version}/tesseract-ocr-w64-setup-${version}.exe"
        Write-Host "Trying to download Tesseract ${version} from $TESSERACT_URL ..."
        
        try {
            $response = Invoke-WebRequest -Uri $TESSERACT_URL -OutFile $INSTALLER_PATH -UseBasicParsing -ErrorAction Stop
            # Check if we got HTML (404 page) instead of the installer
            $fileInfo = Get-Item $INSTALLER_PATH
            if ($fileInfo.Length -lt 1000000) {  # Installer should be > 1MB
                Write-Host "Downloaded file is too small (${fileInfo.Length} bytes), likely an error page. Trying next version..."
                Remove-Item -Path $INSTALLER_PATH -Force -ErrorAction SilentlyContinue
                continue
            }
            Write-Host "Download complete: $INSTALLER_PATH (Size: $($fileInfo.Length) bytes)"
            $DOWNLOAD_SUCCESS = $true
            $TESSERACT_VERSION = $version
            break
        } catch {
            Write-Host "Failed to download version ${version}: $_"
            Remove-Item -Path $INSTALLER_PATH -Force -ErrorAction SilentlyContinue
            continue
        }
    }
}

if (-not $DOWNLOAD_SUCCESS) {
    Write-Error "Failed to install Tesseract using all available methods."
    Write-Error "Tried:"
    Write-Error "  1. winget install UB-Mannheim.TesseractOCR"
    Write-Error "  2. choco install tesseract"
    Write-Error "  3. Direct download (versions: $($TESSERACT_VERSIONS -join ', '))"
    Write-Error ""
    Write-Error "For CI environments, winget should be available on Windows runners."
    Write-Error "Please check the GitHub Actions logs for specific error messages."
    exit 1
}

# Only run installer if we downloaded it directly (not if we used winget/Chocolatey)
if ($DOWNLOAD_SUCCESS -and $TESSERACT_VERSION) {
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
}

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

