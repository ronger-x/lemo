# 安装代码签名证书到系统
# 将证书安装到受信任的根证书颁发机构和受信任的发布者

Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  安装代码签名证书到系统" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# 检查是否以管理员身份运行
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "❌ 此脚本需要管理员权限" -ForegroundColor Red
    Write-Host "   请右键选择 '以管理员身份运行 PowerShell' 后重试" -ForegroundColor Yellow
    Write-Host ""
    pause
    exit 1
}

$cerFile = "lemo-codesign.cer"

# 检查证书文件
if (-not (Test-Path $cerFile)) {
    Write-Host "❌ 找不到证书文件: $cerFile" -ForegroundColor Red
    Write-Host ""
    Write-Host "请先运行 .\create-certificate.ps1 创建证书" -ForegroundColor Yellow
    Write-Host ""
    pause
    exit 1
}

Write-Host "📋 将要安装的证书: $cerFile" -ForegroundColor Cyan
Write-Host ""

# 读取证书信息
try {
    $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($cerFile)
    Write-Host "证书信息:" -ForegroundColor Cyan
    Write-Host "  主题:   $($cert.Subject)" -ForegroundColor White
    Write-Host "  颁发者: $($cert.Issuer)" -ForegroundColor White
    Write-Host "  有效期: $($cert.NotBefore.ToString('yyyy-MM-dd')) 到 $($cert.NotAfter.ToString('yyyy-MM-dd'))" -ForegroundColor White
    Write-Host "  指纹:   $($cert.Thumbprint)" -ForegroundColor White
    Write-Host ""
} catch {
    Write-Host "❌ 无法读取证书: $_" -ForegroundColor Red
    pause
    exit 1
}

$confirm = Read-Host "是否安装此证书到系统? (Y/N)"
if ($confirm -ne "Y" -and $confirm -ne "y") {
    Write-Host "已取消" -ForegroundColor Gray
    exit 0
}

Write-Host ""
Write-Host "🔐 正在安装证书..." -ForegroundColor Cyan
Write-Host ""

try {
    # 安装到受信任的根证书颁发机构
    Write-Host "📦 安装到受信任的根证书颁发机构..." -ForegroundColor Cyan
    Import-Certificate -FilePath $cerFile -CertStoreLocation "Cert:\LocalMachine\Root" | Out-Null
    Write-Host "✅ 已安装到: Cert:\LocalMachine\Root" -ForegroundColor Green
    Write-Host ""
    
    # 安装到受信任的发布者
    Write-Host "📦 安装到受信任的发布者..." -ForegroundColor Cyan
    Import-Certificate -FilePath $cerFile -CertStoreLocation "Cert:\LocalMachine\TrustedPublisher" | Out-Null
    Write-Host "✅ 已安装到: Cert:\LocalMachine\TrustedPublisher" -ForegroundColor Green
    Write-Host ""
    
    Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "✅ 证书安装成功！" -ForegroundColor Green
    Write-Host "═══════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "📝 现在你可以:" -ForegroundColor Cyan
    Write-Host "  1. 编译项目: cargo build --release" -ForegroundColor White
    Write-Host "  2. 签名程序: .\sign-release.ps1" -ForegroundColor White
    Write-Host ""
    Write-Host "签名后，UAC 弹窗将显示:" -ForegroundColor Cyan
    Write-Host "  已验证的发布者: $certName" -ForegroundColor Yellow
    Write-Host "  而不是 '未知'" -ForegroundColor Gray
    Write-Host ""
    
} catch {
    Write-Host "❌ 安装证书失败: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "可能的原因:" -ForegroundColor Yellow
    Write-Host "  - 权限不足（确保以管理员身份运行）" -ForegroundColor Gray
    Write-Host "  - 证书格式错误" -ForegroundColor Gray
    Write-Host ""
    pause
    exit 1
}

Write-Host "按任意键退出..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
