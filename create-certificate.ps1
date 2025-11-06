# 创建自签名代码签名证书
# 用于签名 lemo.exe，解决 UAC "未知发布者" 问题

Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  创建自签名代码签名证书" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# 检查是否以管理员身份运行
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "⚠️  此脚本需要管理员权限" -ForegroundColor Yellow
    Write-Host "   请右键选择 '以管理员身份运行 PowerShell' 后重试" -ForegroundColor Gray
    Write-Host ""
    pause
    exit 1
}

# 配置信息
$certName = "ronger.io"
$certOrg = "ronger.io"
$certCountry = "CN"
$certYears = 5
$pfxPassword = "Lemo2025!"
$pfxFile = "lemo-codesign.pfx"
$cerFile = "lemo-codesign.cer"

Write-Host "📋 证书配置信息:" -ForegroundColor Cyan
Write-Host "  通用名称 (CN):  $certName" -ForegroundColor White
Write-Host "  组织 (O):       $certOrg" -ForegroundColor White
Write-Host "  国家 (C):       $certCountry" -ForegroundColor White
Write-Host "  有效期:         $certYears 年" -ForegroundColor White
Write-Host "  密码:           $pfxPassword" -ForegroundColor White
Write-Host ""

$confirm = Read-Host "是否使用以上配置创建证书? (Y/N)"
if ($confirm -ne "Y" -and $confirm -ne "y") {
    Write-Host "已取消" -ForegroundColor Gray
    exit 0
}

Write-Host ""
Write-Host "🔐 正在创建证书..." -ForegroundColor Cyan

try {
    # 创建自签名证书
    $cert = New-SelfSignedCertificate `
        -Type CodeSigningCert `
        -Subject "CN=$certName, O=$certOrg, C=$certCountry" `
        -KeyAlgorithm RSA `
        -KeyLength 2048 `
        -Provider "Microsoft Enhanced RSA and AES Cryptographic Provider" `
        -CertStoreLocation "Cert:\CurrentUser\My" `
        -NotAfter (Get-Date).AddYears($certYears) `
        -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3")
    
    Write-Host "✅ 证书创建成功" -ForegroundColor Green
    Write-Host "   指纹: $($cert.Thumbprint)" -ForegroundColor Gray
    Write-Host "   主题: $($cert.Subject)" -ForegroundColor Gray
    Write-Host "   有效期: $($cert.NotBefore.ToString('yyyy-MM-dd')) 到 $($cert.NotAfter.ToString('yyyy-MM-dd'))" -ForegroundColor Gray
    Write-Host ""
    
    # 导出 PFX（包含私钥）
    Write-Host "📦 正在导出证书（包含私钥）..." -ForegroundColor Cyan
    $password = ConvertTo-SecureString -String $pfxPassword -Force -AsPlainText
    Export-PfxCertificate -Cert $cert -FilePath $pfxFile -Password $password | Out-Null
    Write-Host "✅ 已导出: $pfxFile" -ForegroundColor Green
    Write-Host "   ⚠️  此文件包含私钥，请妥善保管！" -ForegroundColor Yellow
    Write-Host ""
    
    # 导出 CER（公钥）
    Write-Host "📦 正在导出公钥证书..." -ForegroundColor Cyan
    Export-Certificate -Cert $cert -FilePath $cerFile | Out-Null
    Write-Host "✅ 已导出: $cerFile" -ForegroundColor Green
    Write-Host "   此文件可以分发给其他用户安装" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "✅ 证书创建完成！" -ForegroundColor Green
    Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "📁 生成的文件:" -ForegroundColor Cyan
    Write-Host "  $pfxFile - 代码签名证书（包含私钥，用于签名）" -ForegroundColor White
    Write-Host "  $cerFile - 公钥证书（可分发给用户安装）" -ForegroundColor White
    Write-Host ""
    
    Write-Host "🔐 证书密码:" -ForegroundColor Cyan
    Write-Host "  $pfxPassword" -ForegroundColor Yellow
    Write-Host "  （签名时需要此密码）" -ForegroundColor Gray
    Write-Host ""
    
    Write-Host "📝 下一步:" -ForegroundColor Cyan
    Write-Host "  1. 运行 .\install-certificate.ps1 安装证书到系统" -ForegroundColor White
    Write-Host "  2. 运行 .\sign-release.ps1 签名编译后的 lemo.exe" -ForegroundColor White
    Write-Host ""
    
} catch {
    Write-Host "❌ 创建证书失败: $_" -ForegroundColor Red
    exit 1
}

Write-Host "按任意键退出..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
