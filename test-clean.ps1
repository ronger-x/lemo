# Lemo Clean Temp Test Script
# This script demonstrates the clean-temp functionality

Write-Host "=== Lemo Clean Temp Function Test ===" -ForegroundColor Green
Write-Host ""

Write-Host "Available cleaning options:" -ForegroundColor Cyan
Write-Host "1. Clean Windows system temp only (requires admin)" -ForegroundColor White
Write-Host "   Command: lemo clean-temp" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Clean all temp files including user temp and browser cache (requires admin)" -ForegroundColor White
Write-Host "   Command: lemo clean-temp --include-user" -ForegroundColor Gray
Write-Host ""

Write-Host "Features:" -ForegroundColor Yellow
Write-Host "  - Cleans C:\Windows\Temp" -ForegroundColor White
Write-Host "  - Cleans user TEMP folder (with -i flag)" -ForegroundColor White
Write-Host "  - Cleans Chrome cache (with -i flag)" -ForegroundColor White
Write-Host "  - Cleans Edge cache (with -i flag)" -ForegroundColor White
Write-Host "  - Shows detailed deletion logs" -ForegroundColor White
Write-Host "  - Displays total space freed" -ForegroundColor White
Write-Host "  - Automatic admin privilege elevation" -ForegroundColor White
Write-Host ""

Write-Host "Press Ctrl+C to cancel or any other key to run: lemo clean-temp" -ForegroundColor Yellow
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

Write-Host ""
& ".\target\release\lemo.exe" clean-temp
