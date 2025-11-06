# 签名 Lemo 应用程序
# 使用代码签名证书对编译后的 lemo.exe 进行数字签名

param(
    [string]$ExePath = ".\target\release\lemo.exe",
    [string]$CertPath = ".\lemo-codesign.pfx",
    [string]$Password = "Lemo2025!"
)

Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Lemo 应用程序签名工具" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# 检查可执行文件
if (-not (Test-Path $ExePath)) {
    Write-Host "❌ 找不到文件: $ExePath" -ForegroundColor Red
    Write-Host ""
    Write-Host "请先编译项目:" -ForegroundColor Yellow
    Write-Host "  cargo build --release" -ForegroundColor White
    Write-Host ""
    pause
    exit 1
}

# 检查证书文件
if (-not (Test-Path $CertPath)) {
    Write-Host "❌ 找不到证书: $CertPath" -ForegroundColor Red
    Write-Host ""
    Write-Host "请先运行 .\create-certificate.ps1 创建证书" -ForegroundColor Yellow
    Write-Host ""
    pause
    exit 1
}

Write-Host "📁 目标文件: $ExePath" -ForegroundColor Cyan
Write-Host "🔐 证书文件: $CertPath" -ForegroundColor Cyan
Write-Host ""

# 显示文件信息
$fileInfo = Get-Item $ExePath
Write-Host "文件信息:" -ForegroundColor Gray
Write-Host "  大小:       $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor White
Write-Host "  修改时间:   $($fileInfo.LastWriteTime)" -ForegroundColor White
Write-Host ""

# 检查是否已签名
$existingSig = Get-AuthenticodeSignature -FilePath $ExePath
if ($existingSig.Status -eq "Valid") {
    Write-Host "⚠️  文件已有有效签名:" -ForegroundColor Yellow
    Write-Host "  签名者: $($existingSig.SignerCertificate.Subject)" -ForegroundColor Gray
    Write-Host ""
    $resign = Read-Host "是否重新签名? (Y/N)"
    if ($resign -ne "Y" -and $resign -ne "y") {
        Write-Host "已取消" -ForegroundColor Gray
        exit 0
    }
    Write-Host ""
}

# 加载证书
Write-Host "🔐 正在加载证书..." -ForegroundColor Cyan
try {
    $securePassword = ConvertTo-SecureString -String $Password -Force -AsPlainText
    $cert = Get-PfxCertificate -FilePath $CertPath -Password $securePassword
    
    Write-Host "✅ 证书加载成功" -ForegroundColor Green
    Write-Host "   主题:   $($cert.Subject)" -ForegroundColor Gray
    Write-Host "   颁发者: $($cert.Issuer)" -ForegroundColor Gray
    Write-Host "   有效期: $($cert.NotBefore.ToString('yyyy-MM-dd')) 到 $($cert.NotAfter.ToString('yyyy-MM-dd'))" -ForegroundColor Gray
    Write-Host ""
} catch {
    Write-Host "❌ 无法加载证书: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "可能的原因:" -ForegroundColor Yellow
    Write-Host "  - 证书密码错误" -ForegroundColor Gray
    Write-Host "  - 证书文件损坏" -ForegroundColor Gray
    Write-Host ""
    pause
    exit 1
}

# 签名
Write-Host "✍️  正在签名..." -ForegroundColor Cyan
Write-Host ""

# 时间戳服务器列表（按优先级）
$timestampServers = @(
    "http://timestamp.digicert.com",
    "http://timestamp.sectigo.com",
    "http://timestamp.comodoca.com",
    "http://timestamp.globalsign.com"
)

$signed = $false
foreach ($tsServer in $timestampServers) {
    try {
        Write-Host "  尝试时间戳服务器: $tsServer" -ForegroundColor Gray
        
        $result = Set-AuthenticodeSignature -FilePath $ExePath -Certificate $cert -TimestampServer $tsServer -HashAlgorithm SHA256
        
        if ($result.Status -eq "Valid") {
            $signed = $true
            Write-Host "  ✅ 时间戳添加成功" -ForegroundColor Green
            break
        } else {
            Write-Host "  ⚠️  签名状态: $($result.Status)" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "  ⚠️  失败，尝试下一个服务器..." -ForegroundColor Yellow
        continue
    }
}

if (-not $signed) {
    # 如果所有时间戳服务器都失败，尝试不使用时间戳
    Write-Host ""
    Write-Host "⚠️  所有时间戳服务器都不可用" -ForegroundColor Yellow
    Write-Host "   将不使用时间戳进行签名（签名将在证书过期后失效）" -ForegroundColor Gray
    Write-Host ""
    
    try {
        $result = Set-AuthenticodeSignature -FilePath $ExePath -Certificate $cert -HashAlgorithm SHA256
        $signed = $true
    } catch {
        Write-Host "❌ 签名失败: $_" -ForegroundColor Red
        pause
        exit 1
    }
}

Write-Host ""

if ($signed) {
    Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "✅ 签名成功！" -ForegroundColor Green
    Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host ""
    
    # 验证签名
    $finalSig = Get-AuthenticodeSignature -FilePath $ExePath
    
    Write-Host "📋 签名信息:" -ForegroundColor Cyan
    Write-Host "  状态:       $($finalSig.Status)" -ForegroundColor White
    Write-Host "  签名者:     $($finalSig.SignerCertificate.Subject)" -ForegroundColor White
    Write-Host "  签名算法:   $($finalSig.SignatureType)" -ForegroundColor White
    
    if ($finalSig.TimeStamperCertificate) {
        Write-Host "  时间戳:     $($finalSig.TimeStamperCertificate.Subject)" -ForegroundColor White
    } else {
        Write-Host "  时间戳:     无（签名将在证书过期后失效）" -ForegroundColor Yellow
    }
    
    Write-Host ""
    
    Write-Host "🎉 完成！" -ForegroundColor Green
    Write-Host ""
    Write-Host "现在运行 lemo.exe 时，UAC 弹窗将显示:" -ForegroundColor Cyan
    Write-Host "  已验证的发布者: $($cert.Subject.Split(',')[0].Replace('CN=', ''))" -ForegroundColor Yellow
    Write-Host ""
    
    Write-Host "💡 测试签名:" -ForegroundColor Cyan
    Write-Host "  1. 运行: .\target\release\lemo.exe" -ForegroundColor White
    Write-Host "  2. 查看 UAC 弹窗是否显示你的名字" -ForegroundColor White
    Write-Host ""
    
} else {
    Write-Host "❌ 签名失败" -ForegroundColor Red
    Write-Host ""
}

Write-Host "按任意键退出..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
