@echo off
chcp 65001 >nul
echo 🚀 正在启动 Lemo 安装程序...
echo.
powershell -ExecutionPolicy Bypass -File "%~dp0install.ps1"
pause
