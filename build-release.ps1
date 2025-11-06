# 创建发布包
# 用于打包 Lemo 以便分发

$version = "0.1.0"
$packageName = "lemo-$version-windows-x64"
$packageDir = ".\release-package\$packageName"

Write-Host "📦 创建发布包: $packageName" -ForegroundColor Green

# 检查是否已编译 Release 版本
if (-not (Test-Path ".\target\release\lemo.exe")) {
    Write-Host "❌ 找不到 Release 版本，正在编译..." -ForegroundColor Yellow
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ 编译失败" -ForegroundColor Red
        exit 1
    }
}

# 清理旧的发布目录
if (Test-Path ".\release-package") {
    Remove-Item ".\release-package" -Recurse -Force
}

# 创建发布目录
New-Item -ItemType Directory -Path $packageDir -Force | Out-Null

# 复制文件
Write-Host "📋 复制文件..." -ForegroundColor Cyan
Copy-Item ".\target\release\lemo.exe" -Destination $packageDir
Copy-Item ".\install.ps1" -Destination $packageDir
Copy-Item ".\install.bat" -Destination $packageDir
Copy-Item ".\uninstall.ps1" -Destination $packageDir
Copy-Item ".\uninstall.bat" -Destination $packageDir
Copy-Item ".\README.md" -Destination $packageDir
Copy-Item ".\INSTALL.md" -Destination $packageDir

# 创建一个快速开始文件
$quickStart = @"
# Lemo - Windows 系统工具集

版本: $version

## 快速安装

1. 双击 install.bat（推荐）
   或
   以管理员身份运行 PowerShell 并执行: .\install.ps1

2. 安装完成后，重启终端，即可使用 lemo 命令

## 快速使用

lemo sys-info          # 显示系统信息
lemo fix-icon-cache    # 修复图标缓存
lemo clean-temp        # 清理临时文件

详细文档请查看 README.md 和 INSTALL.md
"@

$quickStart | Out-File -FilePath "$packageDir\快速开始.txt" -Encoding UTF8

# 创建 ZIP 压缩包
Write-Host "🗜️  创建 ZIP 压缩包..." -ForegroundColor Cyan
Compress-Archive -Path $packageDir -DestinationPath ".\release-package\$packageName.zip" -Force

Write-Host ""
Write-Host "✅ 发布包创建成功！" -ForegroundColor Green
Write-Host ""
Write-Host "📂 位置: .\release-package\$packageName.zip" -ForegroundColor Cyan
Write-Host "📊 大小: $([Math]::Round((Get-Item ".\release-package\$packageName.zip").Length / 1MB, 2)) MB" -ForegroundColor Cyan
Write-Host ""
Write-Host "📦 包含文件:" -ForegroundColor Yellow
Get-ChildItem $packageDir | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor White }
Write-Host ""
