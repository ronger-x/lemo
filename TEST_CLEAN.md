# 清理功能测试指南

## ✅ 已集成的批处理脚本功能

基于提供的批处理脚本，`clean_temp` 函数现在清理以下内容：

### 🗑️ 清理范围

#### 1. Windows 系统目录
- ✅ `C:\Windows\Temp` - Windows 临时文件
- ✅ `C:\Windows\Prefetch` - 预读取缓存
- ✅ `C:\Windows\*.bak` - 备份文件（通过扩展名清理功能）

#### 2. 系统驱动器临时文件（递归搜索）
- ✅ `*.tmp` - 临时文件
- ✅ `*.log` - 日志文件
- ✅ `*.gid` - Windows 帮助文件索引
- ✅ `*.chk` - 磁盘检查碎片文件
- ✅ `*.old` - 旧文件备份
- ✅ `*.bak` - 备份文件
- ✅ `*._mp` - 临时文件

#### 3. 用户目录（需要 `--include-user` 或 `-i` 参数）
- ✅ `%TEMP%` - 用户临时文件夹
- ✅ `%USERPROFILE%\Cookies` - Cookie 文件
- ✅ `%USERPROFILE%\Recent` - 最近使用的文件
- ✅ `%USERPROFILE%\Local Settings\Temporary Internet Files` - IE 临时文件
- ✅ `%USERPROFILE%\Local Settings\Temp` - 本地临时文件

### 🔒 安全保护

自动跳过以下关键系统目录（避免损坏系统）：
- ❌ Windows
- ❌ Program Files
- ❌ Program Files (x86)
- ❌ System Volume Information
- ❌ $Recycle.Bin

## 🧪 测试命令

### 测试 1: 仅清理系统文件（不需要管理员权限的部分）
```powershell
.\target\release\lemo.exe clean-temp
```

### 测试 2: 清理系统 + 用户文件（推荐，需要管理员权限）
```powershell
.\target\release\lemo.exe clean-temp --include-user
```
或简写：
```powershell
.\target\release\lemo.exe clean-temp -i
```

### 测试 3: TUI 模式测试（需要管理员权限）
```powershell
# 以管理员身份运行
.\target\release\lemo.exe

# 然后在 TUI 中：
# 1. 使用 ↓ 键选择 "🧹 Clean Temp Files"
# 2. 按 Enter 执行
# 3. 使用 ↑/↓ 键滚动查看清理日志
# 4. 按 Q 返回主菜单
```

## 📊 预期输出

### 清理过程示例
```
🧹 Cleaning temporary files...
═══════════════════════════════════════════════════

📁 Cleaning Windows temp directory: C:\Windows\Temp
   ✅ Deleted: temp_file_1.tmp
   ✅ Deleted: temp_file_2.tmp
   ... and 150 more items deleted
   Deleted: 155 items, Skipped: 3, Freed: 45.23 MB

📁 Cleaning Windows prefetch: C:\Windows\Prefetch
   ✅ Deleted: CHROME.EXE-12345678.pf
   ✅ Deleted: NOTEPAD.EXE-87654321.pf
   ... and 89 more items deleted
   Deleted: 92 items, Skipped: 0, Freed: 12.45 MB

📁 Cleaning system drive temporary files...
   ✅ Deleted: old_backup.bak
   ✅ Deleted: system.log
   ✅ Deleted: temp_data.tmp
   ... and 234 more items deleted
   Deleted: 237 items, Skipped: 15, Freed: 78.90 MB

📁 Cleaning user temp directory: C:\Users\YourName\AppData\Local\Temp
   ✅ Deleted: chrome_cache_1.tmp
   ... and 456 more items deleted
   Deleted: 459 items, Skipped: 23, Freed: 234.56 MB

═══════════════════════════════════════════════════
📊 Cleaning summary:
   Total deleted: 943 items
   Total skipped: 41 items
   Freed space: 371.14 MB (0.36 GB)
═══════════════════════════════════════════════════
✨ Cleaning completed!
```

## 🎯 新增功能对比

### 原批处理脚本 vs 新 Rust 实现

| 功能 | 批处理脚本 | Rust 实现 | 优势 |
|------|-----------|----------|------|
| 清理速度 | 慢 | 快 | ✅ 多线程优化 |
| 错误处理 | 基础 | 完善 | ✅ 详细的错误报告 |
| 进度显示 | 无 | 有 | ✅ 实时显示清理进度 |
| 空间统计 | 无 | 有 | ✅ 显示释放的空间大小 |
| 递归清理 | 有限 | 完整 | ✅ 智能跳过系统目录 |
| 交互模式 | 无 | TUI | ✅ 可视化界面，可滚动查看 |
| 日志详情 | 基础 | 详细 | ✅ 显示每个文件的处理状态 |

## ⚠️ 注意事项

1. **管理员权限**：清理系统文件需要管理员权限
2. **安全性**：程序会自动跳过关键系统目录
3. **可撤销性**：删除的文件无法恢复，请谨慎使用
4. **首次运行**：建议先不加 `-i` 参数测试，确认安全后再清理用户文件

## 🚀 快速开始

**推荐使用方式**（最安全）：
```powershell
# 1. 先测试基础清理
.\target\release\lemo.exe clean-temp

# 2. 确认无误后，清理所有
.\target\release\lemo.exe clean-temp -i

# 3. 或使用 TUI 可视化模式
.\target\release\lemo.exe
```
