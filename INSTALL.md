# Lemo 快速安装指南

## 📦 安装步骤

### 1️⃣ 编译项目（首次安装需要）

如果你已经下载了源代码，首先需要编译：

```powershell
cargo build --release
```

### 2️⃣ 安装到系统

**方法 A：双击批处理文件（最简单）**

直接双击 `install.bat` 文件，在 UAC 提示时点击"是"。

**方法 B：使用 PowerShell**

在项目目录下以管理员身份运行：

```powershell
powershell -ExecutionPolicy Bypass -File install.ps1
```

### 3️⃣ 验证安装

安装完成后，打开**新的**命令行窗口（重要！），输入：

```powershell
lemo --version
```

如果显示版本号，说明安装成功！

## 🎯 快速使用

```powershell
# 显示帮助
lemo --help

# 查看系统信息
lemo sys-info

# 修复图标缓存
lemo fix-icon-cache

# 清理临时文件
lemo clean-temp
```

## ⚠️ 常见问题

### Q: 运行 `lemo` 提示"找不到命令"？

**A:** 可能的原因：
1. 没有重启终端窗口（安装后必须打开新的终端）
2. 没有以管理员权限运行安装脚本
3. 需要重新登录 Windows

### Q: 需要管理员权限吗？

**A:** 
- 安装/卸载时：**需要**管理员权限（用于修改系统 PATH）
- 运行 `sys-info`：**不需要**管理员权限
- 运行 `fix-icon-cache` 和 `clean-temp`：**需要**管理员权限（程序会自动请求提权）

### Q: 如何卸载？

**A:** 双击 `uninstall.bat` 或运行：

```powershell
powershell -ExecutionPolicy Bypass -File uninstall.ps1
```

## 📍 安装位置

默认安装到：`%LOCALAPPDATA%\lemo`（通常是 `C:\Users\你的用户名\AppData\Local\lemo`）

## 🔄 更新

如果需要更新到新版本：

1. 重新编译：`cargo build --release`
2. 重新运行安装脚本

安装脚本会自动覆盖旧版本。
