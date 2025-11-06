# Lemo Installation Script
# Requires Administrator privileges

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\lemo"
)

Write-Host "Starting Lemo installation..." -ForegroundColor Green

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "Administrator privileges required to modify system PATH" -ForegroundColor Yellow
    Write-Host "Attempting to restart with elevated privileges..." -ForegroundColor Yellow
    
    Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`" -InstallPath `"$InstallPath`"" -Verb RunAs
    exit
}

# Create installation directory
Write-Host "Creating installation directory: $InstallPath" -ForegroundColor Cyan
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}

# Copy executable file
Write-Host "Copying executable file..." -ForegroundColor Cyan
$exePath = Join-Path $PSScriptRoot "target\release\lemo.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "ERROR: lemo.exe not found. Please run 'cargo build --release' first" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Copy-Item $exePath -Destination $InstallPath -Force

# Add to system PATH
Write-Host "Adding to system PATH..." -ForegroundColor Cyan

# Get current system PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")

# Check if already in PATH
if ($currentPath -notlike "*$InstallPath*") {
    $newPath = "$currentPath;$InstallPath"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
    Write-Host "Successfully added to system PATH" -ForegroundColor Green
} else {
    Write-Host "Already in system PATH, skipping" -ForegroundColor Green
}

# Refresh current session PATH
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")

Write-Host ""
Write-Host "====================================" -ForegroundColor Green
Write-Host "Installation completed successfully!" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green
Write-Host ""
Write-Host "Installation location: $InstallPath" -ForegroundColor Cyan
Write-Host ""
Write-Host "Usage:" -ForegroundColor Yellow
Write-Host "  lemo fix-icon-cache    # Fix icon cache" -ForegroundColor White
Write-Host "  lemo clean-temp        # Clean temporary files" -ForegroundColor White
Write-Host "  lemo sys-info          # Display system information" -ForegroundColor White
Write-Host ""
Write-Host "NOTE: You may need to restart your terminal or re-login for changes to take effect" -ForegroundColor Yellow
Write-Host ""

Read-Host "Press Enter to exit"
