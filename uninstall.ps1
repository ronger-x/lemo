# Lemo Uninstallation Script
# Requires Administrator privileges

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\lemo"
)

Write-Host "Starting Lemo uninstallation..." -ForegroundColor Yellow

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "Administrator privileges required to modify system PATH" -ForegroundColor Yellow
    Write-Host "Attempting to restart with elevated privileges..." -ForegroundColor Yellow
    
    Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`" -InstallPath `"$InstallPath`"" -Verb RunAs
    exit
}

# Remove from system PATH
Write-Host "Removing from system PATH..." -ForegroundColor Cyan

$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")

if ($currentPath -like "*$InstallPath*") {
    $newPath = ($currentPath -split ';' | Where-Object { $_ -ne $InstallPath }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
    Write-Host "Successfully removed from system PATH" -ForegroundColor Green
} else {
    Write-Host "Not in system PATH, skipping" -ForegroundColor Green
}

# Delete installation directory
if (Test-Path $InstallPath) {
    Write-Host "Deleting installation directory: $InstallPath" -ForegroundColor Cyan
    Remove-Item -Path $InstallPath -Recurse -Force
    Write-Host "Successfully deleted installation directory" -ForegroundColor Green
} else {
    Write-Host "Installation directory does not exist, skipping" -ForegroundColor Green
}

Write-Host ""
Write-Host "====================================" -ForegroundColor Green
Write-Host "Uninstallation completed!" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green
Write-Host ""

Read-Host "Press Enter to exit"
