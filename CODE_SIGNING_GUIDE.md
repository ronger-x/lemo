# Windows UAC "未知发布者" 解决方案

## 🎯 问题说明

当运行 `lemo.exe` 需要管理员权限时，Windows 11 的用户账户控制（UAC）弹窗会显示：

```
用户账户控制
你要允许此应用对你的设备进行更改吗？

Lemo - Windows System Toolkit
已验证的发布者: 未知          ← 这里显示"未知"
文件原始位置: E:\workspace\lemo\target\release\lemo.exe

[是(Y)]  [否(N)]
```

**原因：** 应用程序没有进行数字签名。

## ✅ 解决方案

### 方案 1: 自签名证书（开发/测试环境）⭐ 推荐用于个人使用

这种方式**免费**，适合个人开发者和内部使用，但用户需要信任你的证书。

#### 步骤 1: 创建自签名证书

```powershell
# 以管理员身份运行 PowerShell

# 创建自签名证书
$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject "CN=ronger.io, O=ronger.io, C=CN" `
    -KeyAlgorithm RSA `
    -KeyLength 2048 `
    -Provider "Microsoft Enhanced RSA and AES Cryptographic Provider" `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -NotAfter (Get-Date).AddYears(5) `
    -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3")

# 导出证书（包含私钥）
$password = ConvertTo-SecureString -String "YourPassword123!" -Force -AsPlainText
Export-PfxCertificate -Cert $cert -FilePath ".\lemo-codesign.pfx" -Password $password

# 导出公钥证书（用于分发给其他用户）
Export-Certificate -Cert $cert -FilePath ".\lemo-codesign.cer"

Write-Host "✅ 证书创建成功！" -ForegroundColor Green
Write-Host "   私钥证书: lemo-codesign.pfx (用于签名，请妥善保管)" -ForegroundColor Yellow
Write-Host "   公钥证书: lemo-codesign.cer (可分发给用户安装)" -ForegroundColor Yellow
```

**参数说明：**
- `CN=ronger.io` - 通用名称（Common Name），通常是你的名字或组织名
- `O=ronger.io` - 组织名称（Organization）
- `C=CN` - 国家代码（中国）
- `-NotAfter` - 证书有效期（这里设置为5年）

#### 步骤 2: 安装证书到受信任的根证书颁发机构

```powershell
# 以管理员身份运行

# 导入证书到受信任的根证书颁发机构
Import-Certificate -FilePath ".\lemo-codesign.cer" -CertStoreLocation "Cert:\LocalMachine\Root"

# 导入证书到受信任的发布者
Import-Certificate -FilePath ".\lemo-codesign.cer" -CertStoreLocation "Cert:\LocalMachine\TrustedPublisher"

Write-Host "✅ 证书已安装到系统信任列表" -ForegroundColor Green
```

#### 步骤 3: 签名应用程序

```powershell
# 方法 1: 使用 PFX 文件签名
$password = ConvertTo-SecureString -String "YourPassword123!" -Force -AsPlainText
$cert = Get-PfxCertificate -FilePath ".\lemo-codesign.pfx" -Password $password

Set-AuthenticodeSignature -FilePath ".\target\release\lemo.exe" -Certificate $cert -TimestampServer "http://timestamp.digicert.com"

# 方法 2: 使用已安装的证书签名
$cert = Get-ChildItem -Path "Cert:\CurrentUser\My" -CodeSigningCert | Where-Object {$_.Subject -like "*ronger.io*"}
Set-AuthenticodeSignature -FilePath ".\target\release\lemo.exe" -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
```

**时间戳服务器（重要）：**
- `http://timestamp.digicert.com` - DigiCert 时间戳
- `http://timestamp.sectigo.com` - Sectigo 时间戳
- `http://timestamp.comodoca.com` - Comodo 时间戳

时间戳可以让签名在证书过期后仍然有效。

#### 步骤 4: 验证签名

```powershell
# 查看签名信息
Get-AuthenticodeSignature -FilePath ".\target\release\lemo.exe" | Format-List

# 检查签名状态
$sig = Get-AuthenticodeSignature -FilePath ".\target\release\lemo.exe"
if ($sig.Status -eq "Valid") {
    Write-Host "✅ 签名有效！" -ForegroundColor Green
} else {
    Write-Host "❌ 签名状态: $($sig.Status)" -ForegroundColor Red
}
```

#### ⚠️ 自签名证书的限制

- ✅ **优点：** 免费、快速、适合个人使用
- ❌ **缺点：** 
  - 需要用户手动安装证书才能信任
  - UAC 仍会显示黄色警告（但会显示你的名字而不是"未知"）
  - 不适合公开分发的商业软件

---

### 方案 2: 商业代码签名证书（生产环境）⭐⭐⭐ 推荐用于公开分发

购买受信任的 CA 颁发的代码签名证书，UAC 会显示绿色盾牌和你的公司名。

#### 证书提供商

| 提供商 | 价格/年 | 特点 |
|--------|---------|------|
| **DigiCert** | $300-500 | 最受信任，SmartScreen 信誉累积快 |
| **Sectigo (Comodo)** | $200-400 | 性价比高，广泛兼容 |
| **GlobalSign** | $250-450 | 国际认可度高 |
| **Certum** | $150-300 | 欧洲品牌，价格实惠 |
| **国内CA（如天威诚信）** | ¥1000-3000 | 国产证书，政府项目推荐 |

#### 证书类型

1. **标准代码签名证书（Standard Code Signing）**
   - 价格：$200-400/年
   - 验证：组织验证（OV）
   - 需要：公司注册文件、营业执照
   - 适用：公司、组织

2. **EV 代码签名证书（Extended Validation）**
   - 价格：$400-600/年
   - 验证：扩展验证（EV）
   - 需要：更严格的公司验证 + USB 硬件令牌
   - 优势：立即获得 Windows SmartScreen 信誉，UAC 显示绿色
   - 适用：商业软件、公开分发

3. **个人代码签名证书**
   - 某些 CA 提供（如 Certum）
   - 价格：$100-200/年
   - 验证：身份证、护照
   - 适用：个人开发者

#### 购买流程

1. **选择证书提供商和类型**
2. **提交验证材料**
   - 公司：营业执照、组织机构代码、法人身份证
   - 个人：身份证、护照
3. **等待验证**（1-7个工作日）
4. **下载证书**（.pfx 或 .p12 格式）
5. **签名应用程序**

#### 使用商业证书签名

```powershell
# 安装 PFX 证书（如果未安装）
Import-PfxCertificate -FilePath "C:\path\to\your-cert.pfx" -CertStoreLocation "Cert:\CurrentUser\My" -Password $password

# 签名应用程序
$cert = Get-ChildItem -Path "Cert:\CurrentUser\My" -CodeSigningCert
Set-AuthenticodeSignature -FilePath ".\target\release\lemo.exe" -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
```

或使用 `signtool.exe`（Windows SDK 工具）：

```cmd
signtool sign /f "your-cert.pfx" /p "password" /tr "http://timestamp.digicert.com" /td SHA256 /fd SHA256 "lemo.exe"
```

---

### 方案 3: 开源项目免费签名

如果 Lemo 是开源项目，可以申请免费代码签名：

#### SignPath Foundation（免费）

- 网址：https://signpath.org/
- 适用：开源项目
- 要求：项目托管在 GitHub 等公开平台
- 流程：
  1. 在 GitHub 上申请
  2. 集成到 CI/CD 流程
  3. 自动签名构建产物

---

## 🛠️ 自动化签名脚本

创建一个自动签名脚本 `sign-release.ps1`：

```powershell
# 签名发布版本
param(
    [string]$CertPath = ".\lemo-codesign.pfx",
    [string]$Password = "",
    [string]$ExePath = ".\target\release\lemo.exe"
)

Write-Host "🔐 开始签名流程..." -ForegroundColor Cyan
Write-Host ""

# 检查文件
if (-not (Test-Path $ExePath)) {
    Write-Host "❌ 找不到: $ExePath" -ForegroundColor Red
    Write-Host "请先编译: cargo build --release" -ForegroundColor Yellow
    exit 1
}

if (-not (Test-Path $CertPath)) {
    Write-Host "❌ 找不到证书: $CertPath" -ForegroundColor Red
    exit 1
}

# 输入密码
if ([string]::IsNullOrEmpty($Password)) {
    $securePassword = Read-Host "请输入证书密码" -AsSecureString
} else {
    $securePassword = ConvertTo-SecureString -String $Password -Force -AsPlainText
}

# 加载证书
try {
    $cert = Get-PfxCertificate -FilePath $CertPath -Password $securePassword
    Write-Host "✅ 证书加载成功" -ForegroundColor Green
    Write-Host "   主题: $($cert.Subject)" -ForegroundColor Gray
    Write-Host "   有效期: $($cert.NotBefore) 到 $($cert.NotAfter)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 无法加载证书: $_" -ForegroundColor Red
    exit 1
}

# 签名
Write-Host ""
Write-Host "📝 正在签名..." -ForegroundColor Cyan
try {
    $result = Set-AuthenticodeSignature -FilePath $ExePath -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
    
    if ($result.Status -eq "Valid") {
        Write-Host "✅ 签名成功！" -ForegroundColor Green
        Write-Host ""
        Write-Host "签名信息:" -ForegroundColor Cyan
        Write-Host "  状态: $($result.Status)" -ForegroundColor White
        Write-Host "  签名者: $($result.SignerCertificate.Subject)" -ForegroundColor White
        Write-Host "  时间戳: $($result.TimeStamperCertificate.Subject)" -ForegroundColor White
    } else {
        Write-Host "⚠️  签名状态: $($result.Status)" -ForegroundColor Yellow
        Write-Host "  消息: $($result.StatusMessage)" -ForegroundColor Gray
    }
} catch {
    Write-Host "❌ 签名失败: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎉 完成！" -ForegroundColor Green
```

---

## 📊 各方案对比

| 方案 | 成本 | 信任度 | UAC 显示 | 适用场景 |
|------|------|--------|----------|----------|
| **自签名证书** | 免费 | ⭐⭐ | 黄色，显示你的名字 | 个人使用、内部测试 |
| **标准代码签名** | $200-400/年 | ⭐⭐⭐⭐ | 蓝色，显示公司名 | 小型商业软件 |
| **EV 代码签名** | $400-600/年 | ⭐⭐⭐⭐⭐ | 绿色，立即受信任 | 公开分发、企业软件 |
| **开源免费签名** | 免费 | ⭐⭐⭐⭐ | 蓝色，显示组织名 | 开源项目 |

---

## 🎯 推荐方案

### 个人开发/学习项目
→ **使用自签名证书**
- 免费快速
- 在自己电脑上完全受信任
- 适合个人使用和测试

### 小规模分发（朋友、小团队）
→ **使用自签名证书 + 提供安装说明**
- 提供证书安装脚本
- 用户安装后完全信任
- 成本为零

### 公开分发的免费软件
→ **购买标准代码签名证书** 或 **申请开源免费签名**
- 提升用户信任度
- 减少误报和警告
- 累积 SmartScreen 信誉

### 商业软件
→ **购买 EV 代码签名证书**
- 最高信任度
- 立即获得 Windows SmartScreen 白名单
- 专业形象

---

## 📋 快速开始（自签名方案）

对于 Lemo 项目，建议使用自签名证书用于个人使用：

```powershell
# 1. 创建证书（运行一次）
.\create-certificate.ps1

# 2. 安装证书到系统（运行一次）
.\install-certificate.ps1

# 3. 编译项目
cargo build --release

# 4. 签名应用
.\sign-release.ps1
```

---

## ❓ 常见问题

### Q: 自签名后 UAC 还是显示"未知发布者"？

A: 确保已将证书安装到：
- `Cert:\LocalMachine\Root`（受信任的根证书颁发机构）
- `Cert:\LocalMachine\TrustedPublisher`（受信任的发布者）

### Q: 签名后文件运行出错？

A: 检查签名状态：
```powershell
Get-AuthenticodeSignature -FilePath ".\lemo.exe" | Format-List
```
确保 Status 为 "Valid"。

### Q: 能否绕过签名解决 UAC 问题？

A: 不能。Windows 安全机制要求：
- 要么进行数字签名
- 要么用户接受"未知发布者"警告

### Q: 自签名证书过期了怎么办？

A: 如果签名时使用了时间戳服务器，签名在证书过期后仍然有效。否则需要重新签名。

### Q: 如何让其他用户信任我的自签名证书？

A: 分发 `.cer` 证书文件，并提供安装说明：
1. 双击 `lemo-codesign.cer`
2. 点击"安装证书"
3. 选择"本地计算机" → "将所有的证书都放入下列存储"
4. 选择"受信任的根证书颁发机构"
5. 完成安装

---

## 📚 参考资料

- [Microsoft: Code Signing Best Practices](https://docs.microsoft.com/en-us/windows-hardware/drivers/dashboard/code-signing-cert-manage)
- [About Code Signing](https://docs.microsoft.com/en-us/windows/win32/seccrypto/cryptography-tools)
- [SignTool Documentation](https://docs.microsoft.com/en-us/windows/win32/seccrypto/signtool)
