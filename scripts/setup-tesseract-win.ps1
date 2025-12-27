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
    # Check if winget is available
    $wingetVersion = winget --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Winget detected (version: $wingetVersion). Installing Tesseract OCR..."
        
        # Try to install Tesseract via winget
        $installOutput = winget install --id UB-Mannheim.TesseractOCR --silent --accept-package-agreements --accept-source-agreements 2>&1
        $installExitCode = $LASTEXITCODE
        
        if ($installExitCode -eq 0) {
            Write-Host "Winget installation completed. Searching for Tesseract installation..."
            Write-Host "Installation output: $installOutput"
            
            # Wait longer for installation to complete and files to be written
            Start-Sleep -Seconds 5
            
            # Refresh PATH to pick up newly installed executables
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
            
            # Find the installed Tesseract location
            $foundPath = $null
            
            # Try to get install location from winget first
            try {
                $wingetList = winget list --id UB-Mannheim.TesseractOCR 2>&1
                if ($LASTEXITCODE -eq 0 -and $wingetList -match "Installed") {
                    Write-Host "Tesseract is listed as installed by winget"
                    # Try to get install location (winget doesn't directly provide this, but we know it's installed)
                }
            } catch {
                Write-Host "Could not query winget for install status"
            }
            
            # First, check if tesseract is in PATH (most reliable)
            $tesseractInPath = (Get-Command tesseract -ErrorAction SilentlyContinue).Source
            if ($tesseractInPath) {
                $foundPath = Split-Path $tesseractInPath -Parent
                Write-Host "Found Tesseract in PATH at: $foundPath"
            }
            
            # If not in PATH, check common installation locations
            if (-not $foundPath) {
                $commonPaths = @(
                    "${env:ProgramFiles}\Tesseract-OCR",
                    "${env:ProgramFiles(x86)}\Tesseract-OCR"
                )
                
                foreach ($path in $commonPaths) {
                    $tesseractExe = Join-Path $path "tesseract.exe"
                    if (Test-Path $tesseractExe) {
                        $foundPath = $path
                        Write-Host "Found Tesseract in common location: $foundPath"
                        break
                    }
                }
            }
            
            # If still not found, search Program Files recursively (slower but thorough)
            if (-not $foundPath) {
                Write-Host "Searching Program Files recursively for Tesseract..."
                try {
                    # Limit depth to avoid searching too deep (PowerShell 5.1+ supports -Depth)
                    $searchParams = @{
                        Path = "${env:ProgramFiles}"
                        Filter = "tesseract.exe"
                        Recurse = $true
                        ErrorAction = "SilentlyContinue"
                    }
                    # Try with -Depth if available (PowerShell 5.1+), otherwise without
                    try {
                        $foundExe = Get-ChildItem @searchParams -Depth 3 | Select-Object -First 1
                    } catch {
                        $foundExe = Get-ChildItem @searchParams | Select-Object -First 1
                    }
                    
                    if (-not $foundExe -and (Test-Path "${env:ProgramFiles(x86)}")) {
                        $searchParams.Path = "${env:ProgramFiles(x86)}"
                        try {
                            $foundExe = Get-ChildItem @searchParams -Depth 3 | Select-Object -First 1
                        } catch {
                            $foundExe = Get-ChildItem @searchParams | Select-Object -First 1
                        }
                    }
                    
                    if ($foundExe) {
                        $foundPath = $foundExe.DirectoryName
                        Write-Host "Found Tesseract via recursive search: $foundPath"
                    }
                } catch {
                    Write-Host "Recursive search failed: $_"
                }
            }
            
            if ($foundPath) {
                Write-Host "Found Tesseract installation at: $foundPath"
                Write-Host "Copying Tesseract files to bundle directory..."
                Copy-Item -Path "$foundPath\*" -Destination $TESSERACT_DIR -Recurse -Force
                $DOWNLOAD_SUCCESS = $true
                Write-Host "✓ Tesseract installed and bundled via winget"
            } else {
                Write-Host "WARNING: Winget installation reported success but Tesseract location not found."
                Write-Host "This might mean:"
                Write-Host "  1. Installation is still in progress (waiting longer...)"
                Write-Host "  2. Tesseract was installed to an unexpected location"
                Write-Host "  3. Installation actually failed despite exit code 0"
                Write-Host ""
                Write-Host "Installation output was:"
                Write-Host $installOutput
                Write-Host ""
                Write-Host "Attempting to find Tesseract with extended search..."
                
                # Wait a bit more and try again
                Start-Sleep -Seconds 5
                
                # Try one more comprehensive search
                $allDrives = Get-PSDrive -PSProvider FileSystem | Select-Object -ExpandProperty Root
                foreach ($drive in $allDrives) {
                    try {
                        $foundExe = Get-ChildItem -Path $drive -Filter "tesseract.exe" -Recurse -ErrorAction SilentlyContinue -Depth 4 | Select-Object -First 1
                        if ($foundExe) {
                            $foundPath = $foundExe.DirectoryName
                            Write-Host "Found Tesseract via extended search at: $foundPath"
                            Copy-Item -Path "$foundPath\*" -Destination $TESSERACT_DIR -Recurse -Force
                            $DOWNLOAD_SUCCESS = $true
                            Write-Host "✓ Tesseract installed and bundled via winget (extended search)"
                            break
                        }
                    } catch {
                        # Skip drives that can't be accessed
                        continue
                    }
                }
                
                if (-not $DOWNLOAD_SUCCESS) {
                    Write-Host "Extended search also failed. Will try fallback methods..."
                }
            }
        } else {
            Write-Host "Winget installation failed with exit code $installExitCode"
            Write-Host "Installation output: $installOutput"
        }
    } else {
        Write-Host "Winget check failed: $wingetVersion"
    }
} catch {
    Write-Host "Winget not available or error occurred: $_"
}

# Method 2: Try using Chocolatey if winget failed
if (-not $DOWNLOAD_SUCCESS) {
    Write-Host ""
    Write-Host "========================================"
    Write-Host "Attempting fallback: Chocolatey"
    Write-Host "========================================"
    try {
        $chocoCheck = choco --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Chocolatey detected (version: $chocoCheck). Installing Tesseract OCR..."
            $chocoOutput = choco install tesseract -y --no-progress 2>&1
            $chocoExitCode = $LASTEXITCODE
            Write-Host "Chocolatey installation output: $chocoOutput"
            
            if ($chocoExitCode -eq 0) {
                Start-Sleep -Seconds 3
                # Refresh PATH
                $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
                
                # Find the installed Tesseract location
                $tesseractPath = (Get-Command tesseract -ErrorAction SilentlyContinue).Source
                if ($tesseractPath) {
                    $tesseractInstallDir = Split-Path $tesseractPath -Parent
                    Write-Host "Found Tesseract installation at: $tesseractInstallDir"
                    Write-Host "Copying Tesseract files to bundle directory..."
                    Copy-Item -Path "$tesseractInstallDir\*" -Destination $TESSERACT_DIR -Recurse -Force
                    $DOWNLOAD_SUCCESS = $true
                    Write-Host "✓ Tesseract installed and bundled via Chocolatey"
                } else {
                    Write-Host "Chocolatey installation succeeded but tesseract not found in PATH"
                }
            } else {
                Write-Host "Chocolatey installation failed with exit code $chocoExitCode"
            }
        } else {
            Write-Host "Chocolatey not available: $chocoCheck"
        }
    } catch {
        Write-Host "Chocolatey check failed: $_"
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
    Write-Host ""
    Write-Host "========================================"
    Write-Host "ERROR: All installation methods failed"
    Write-Host "========================================"
    Write-Host ""
    Write-Host "Tried the following methods:"
    Write-Host "  1. winget install UB-Mannheim.TesseractOCR"
    Write-Host "  2. choco install tesseract (if available)"
    Write-Host "  3. Direct download (versions: $($TESSERACT_VERSIONS -join ', '))"
    Write-Host ""
    Write-Host "Troubleshooting steps:"
    Write-Host "  - Verify winget is available: winget --version"
    Write-Host "  - Check network connectivity"
    Write-Host "  - Review the detailed output above for specific error messages"
    Write-Host ""
    Write-Host "For GitHub Actions:"
    Write-Host "  - Windows runners should have winget pre-installed"
    Write-Host "  - If winget fails, check if the package ID is correct: UB-Mannheim.TesseractOCR"
    Write-Host "  - Check GitHub Actions logs for network or permission issues"
    Write-Host ""
    Write-Error "Failed to install Tesseract using all available methods. See output above for details."
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

