# 🎯 快速开始指南

## 一键启动

### 方法 1：直接运行（推荐）
```powershell
# 以管理员身份运行 PowerShell，然后执行：
.\target\release\lemo.exe
```

### 方法 2：右键菜单
1. 右键点击 `lemo.exe`
2. 选择 **"以管理员身份运行"**
3. 在 UAC 提示中点击 **"是"**

### 方法 3：PowerShell 提升
```powershell
Start-Process .\target\release\lemo.exe -Verb RunAs
```

---

## 🎨 TUI 界面导航

### 主菜单

```
┌─────────────────────────────────────────┐
│    🍋 Lemo - Windows System Toolkit    │
├─────────────────────────────────────────┤
│ Main Menu                               │
│ > 🔧 Fix Icon Cache                     │  ← 当前选中（黄色高亮）
│   🧹 Clean Temp Files                   │
│   💻 System Info                        │
│   📊 Real-time Monitor                  │
│   🚪 Exit                               │
└─────────────────────────────────────────┘
│ ↑/↓: Navigate | Enter: Execute | Q: Quit│
└─────────────────────────────────────────┘
```

### 键盘操作
- **导航**：
  - `↑` / `k` → 上移
  - `↓` / `j` → 下移
- **执行**：`Enter`
- **退出**：`Q` 或 `Esc`

---

## 📊 实时监控仪表盘

### 界面布局预览

```
┌───────────────────────────────────────────────────────────────┐
│              📊 Real-time System Monitor                      │
├──────────────────────────────┬────────────────────────────────┤
│                              │                                │
│  🔧 CPU: Intel Core i7       │  💿 Disks (按盘符排序)         │
│  ████████████░░░░░░░░ 65.2%  │  C:\  ████████████████ 85%    │
│                              │       213/250 GB               │
│                              │  D:\  ████░░░░░░░░░░░░ 20%    │
│                              │       200/1000 GB              │
├──────────────────────────────┼────────────────────────────────┤
│                              │                                │
│  💾 Memory: 12.8/16.0 GB     │  � GPU & Temperature         │
│  ████████████████░░░ 80.0%   │  🎮 GPU: 65.5°C               │
│                              │                                │
│                              │  CPU Package      72.0°C       │
│                              │  Motherboard      45.0°C       │
├──────────────────────────────┼────────────────────────────────┤
│                              │                                │
│  🌐 Network                  │  ℹ️  System Info              │
│  📡 Ethernet                 │  OS: windows                   │
│     ↓ 1234.56 MB ↑ 567.89 MB │  Arch: x86_64                  │
│  📡 Wi-Fi                    │  Cores: 6 physical, 12 logical │
│     ↓ 89.12 MB   ↑ 45.67 MB  │  Uptime: 2d 5h 23m             │
└──────────────────────────────┴────────────────────────────────┘
│    Press Q/Esc/Enter to return | Updates every 1 second      │
└───────────────────────────────────────────────────────────────┘
```

### 颜色说明

#### CPU 使用率
- 🟢 **绿色**（0-50%）：正常运行
- 🟡 **黄色**（50-80%）：中等负载
- 🔴 **红色**（80-100%）：高负载

#### 内存使用率
- 🟢 **绿色**（0-60%）：充足
- 🟡 **黄色**（60-80%）：警告
- 🔴 **红色**（80-100%）：危险

#### 磁盘使用率
- 🟢 **绿色**（0-70%）：正常
- 🟡 **黄色**（70-90%）：建议清理
- 🔴 **红色**（90-100%）：空间不足

#### 温度监控
- 🟢 **绿色**（< 60°C）：正常
- 🟡 **黄色**（60-80°C）：温暖
- 🔴 **红色**（> 80°C）：过热

---

## 🔧 功能详解

### 1️⃣ 修复图标缓存

**作用**：修复 Windows 图标显示异常、白板图标等问题

**操作步骤**：
1. 选择 **🔧 Fix Icon Cache**
2. 按 `Enter`
3. 等待执行完成
4. 按任意键返回主菜单

**输出示例**：
```
🔧 Fixing icon cache...
⏳ Closing Windows Explorer...
🗑️  Deleted: C:\Users\...\IconCache.db
🗑️  Deleted: C:\Users\...\iconcache_16.db
...
🔄 Restarting Windows Explorer...
✅ Icon cache fixed successfully!
```

### 2️⃣ 清理临时文件

**作用**：深度清理系统垃圾文件，释放磁盘空间

**操作步骤**：
1. 选择 **🧹 Clean Temp Files**
2. 按 `Enter`
3. 等待扫描和清理完成（可能需要 30秒-3分钟）
4. 查看清理结果

**清理内容**：
- ✅ `C:\Windows\Temp` - Windows 临时文件
- ✅ `C:\Windows\Prefetch` - 预读缓存
- ✅ 系统盘上的 `.tmp`, `.log`, `.bak`, `.old` 等文件
- ✅ 用户目录的临时文件（如果选择 `-i` 参数）

**输出示例**：
```
🧹 Cleaning temporary files...
═══════════════════════════════════════════════════

📁 Cleaning Windows temp directory: C:\Windows\Temp
   Deleted: 23 items, Skipped: 5, Freed: 128.45 MB

📁 Cleaning Windows prefetch: C:\Windows\Prefetch
   Deleted: 0 items, Skipped: 0, Freed: 0.00 MB

📁 Scanning system drive for temp files...
   ⏳ Progress: 100 deleted, 45.23 MB freed...
   ⏳ Progress: 200 deleted, 89.67 MB freed...
   ✅ Completed: 256 items deleted, 48 skipped, 103.78 MB freed

═══════════════════════════════════════════════════
📊 Cleaning summary:
   Total deleted: 279 items
   Total skipped: 53 items
   Freed space: 232.23 MB (0.23 GB)
═══════════════════════════════════════════════════
✨ Cleaning completed!
```

### 3️⃣ 系统信息（静态）

**作用**：查看详细的系统信息快照

**操作步骤**：
1. 选择 **💻 System Info**
2. 按 `Enter`
3. 使用 `↑`/`↓` 滚动查看
4. 按 `Q` 返回

**显示内容**：
- 📌 基本信息（OS、架构、用户名、计算机名）
- 🔧 CPU 信息（型号、核心数、频率、使用率）
- 💾 内存信息（总量、已用、可用）
- 💿 磁盘信息（所有驱动器的容量和使用情况）
- ⏱️ 系统运行时间

### 4️⃣ 实时监控（动态）⭐

**作用**：实时监控系统状态，动态刷新

**操作步骤**：
1. 选择 **📊 Real-time Monitor**
2. 按 `Enter`
3. 观察实时数据（每 1 秒刷新）
4. 按 `Q` / `Esc` / `Enter` 返回

**特色功能**：
- ⚡ 动态刷新（1秒间隔）
- 📊 Grid 布局（2x3 六个区域）
- 🎨 彩色进度条（自动变色）
- 🌐 网络流量监控（上传/下载统计）
- 🎮 GPU 温度显示
- 🌡️ 硬件温度监控（需要管理员权限）
- 💿 磁盘按盘符排序（C、D、E...）

**最佳实践**：
- ✅ 以管理员身份运行以显示温度
- ✅ 用于监控系统性能
- ✅ 用于诊断高负载问题

---

## ⚠️ 常见问题

### Q1: 为什么温度显示为空或显示权限提示？
**A**: 温度监控需要管理员权限。解决方法：
```powershell
# 右键以管理员身份运行
Start-Process .\target\release\lemo.exe -Verb RunAs
```

### Q2: 清理临时文件后系统空间没有明显增加？
**A**: 这是正常现象，原因可能是：
- 系统本身就比较干净
- 大部分文件正在被占用（显示为 "Skipped"）
- 建议重启后再次运行清理

### Q3: TUI 界面显示不正常/乱码？
**A**: 确保：
- 使用 PowerShell 或 Windows Terminal（推荐）
- 终端窗口至少 80x24 尺寸
- 字体支持 Unicode（如 Cascadia Code）

### Q4: 程序无法启动，提示权限错误？
**A**: 
1. 确保以管理员身份运行
2. 检查 UAC 设置是否正确
3. 尝试禁用杀毒软件的实时保护（临时）

### Q5: 如何退出实时监控？
**A**: 按以下任意键返回主菜单：
- `Q` 键
- `Esc` 键
- `Enter` 键

---

## 🚀 性能优化建议

### 定期清理（推荐）
```powershell
# 每周执行一次
lemo clean-temp -i
```

### 监控系统健康
```powershell
# 启动 TUI，选择 Real-time Monitor
lemo
# 观察 CPU/内存/磁盘/温度状态
```

### 修复图标问题
```powershell
# 图标显示异常时执行
lemo fix-icon-cache
```

---

## 📝 使用场景示例

### 场景 1：电脑卡顿诊断
1. 启动 `lemo`（TUI 模式）
2. 选择 **📊 Real-time Monitor**
3. 观察 CPU、内存使用率
4. 检查温度是否过高
5. 如果磁盘空间不足，返回执行 **🧹 Clean Temp Files**

### 场景 2：释放磁盘空间
1. 启动 `lemo`
2. 选择 **🧹 Clean Temp Files**
3. 等待清理完成
4. 查看释放的空间大小

### 场景 3：图标显示异常
1. 启动 `lemo`
2. 选择 **🔧 Fix Icon Cache**
3. 等待 Explorer 重启
4. 检查图标是否恢复正常

### 场景 4：系统信息收集
1. 启动 `lemo`
2. 选择 **💻 System Info**
3. 滚动查看所有信息
4. 截图或记录需要的数据

---

## 🎓 进阶用法

### 命令行模式（无 TUI）

适合脚本自动化或 CI/CD：

```powershell
# 修复图标缓存（不重启 Explorer）
lemo fix-icon-cache --restart-explorer false

# 清理临时文件（包含用户目录）
lemo clean-temp --include-user

# 显示系统信息
lemo sys-info
```

### 计划任务自动清理

创建 Windows 计划任务，每周自动清理：

1. 打开"任务计划程序"
2. 创建基本任务
3. 触发器：每周执行
4. 操作：启动程序 → `lemo.exe clean-temp -i`
5. 勾选"使用最高权限运行"

---

## 📚 更多资源

- 📖 [README.md](README.md) - 完整功能文档
- 📊 [REALTIME_MONITOR.md](REALTIME_MONITOR.md) - 实时监控详细说明
- 🛠️ [OPTIMIZATION_DONE.md](OPTIMIZATION_DONE.md) - 优化技术文档

---

**Enjoy using Lemo! 🍋**
