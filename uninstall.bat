@echo off
chcp 65001 >nul
echo 🗑️ 正在启动 Lemo 卸载程序...
echo.
powershell -ExecutionPolicy Bypass -File "%~dp0uninstall.ps1"
pause
