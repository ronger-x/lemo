# Lemo - Windows 系统工具集

一个用 Rust 编写的 Windows 系统维护工具集，现已支持 **Terminal User Interface (TUI)** 交互式界面！

## 功能特性

- 🍋 **全新 TUI 模式** - 漂亮的终端交互界面，支持可滚动查看
- 📊 **实时系统监控** - Grid 布局仪表盘，动态显示 CPU、内存、磁盘、温度
- 🔧 **修复 Windows 图标缓存** - 快速修复图标显示异常
- 🧹 **深度清理系统垃圾** - 清理多种类型的临时文件和缓存
  - Windows Temp 临时文件
  - Windows Prefetch 预读缓存
  - Windows 回收站
  - 系统驱动器上的 *.tmp, *.log, *.bak, *.old 等文件
  - 用户临时文件、Cookies、Recent 文件
  - IE 临时文件和本地缓存
- 📦 **一键安装/卸载** - 自动配置系统 PATH，方便快捷
- ⌨️ **键盘导航** - 支持箭头键和 Vim 风格按键
- 🎨 **彩色界面** - 清晰的视觉反馈和可滚动输出查看器

## 使用方法

### 🎨 TUI 模式（推荐）

直接运行 lemo 进入交互式界面：

```powershell
lemo
```

**TUI 控制键：**
- `↑`/`↓` 或 `j`/`k` - 导航菜单
- `Enter` - 执行选中的操作
- `q` 或 `Esc` - 退出程序

**TUI 菜单选项：**
- 🔧 **Fix Icon Cache** - 修复图标缓存
- 🧹 **Clean Temp Files** - 清理临时文件
- � **Real-time Monitor** - 实时系统监控仪表盘（推荐！）
- � **Install to System** - 安装 lemo 到系统 PATH
- 🗑️ **Uninstall from System** - 从系统卸载 lemo
- ➡️ **Exit** - 退出程序

### 📊 实时系统监控（新功能！）

进入 TUI 模式后，选择 **"Real-time Monitor"** 可查看：

**功能特性：**
- ⚡ **动态刷新** - 每 1 秒自动更新系统状态
- 📊 **Grid 布局** - 左右分栏、上下分割的现代化界面
- 🎨 **彩色进度条** - CPU、内存使用率可视化
- 💿 **多磁盘监控** - 显示所有驱动器的使用情况
- 🌡️ **温度监控** - 硬件温度实时显示（需要管理员权限）
- 📈 **智能配色** - 根据使用率/温度自动变色（绿/黄/红）

**显示内容：**
- 🔧 CPU 使用率（实时进度条 + 型号）
- 💾 内存使用率（实时进度条 + 容量）
- 🌐 网络流量（上传/下载统计，最多3个接口）
- 💿 磁盘使用情况（多驱动器 + ASCII 进度条，按盘符排序）
- 🎮 GPU 温度（显卡温度监控）
- 🌡️ 硬件温度（CPU、主板等传感器）
- ℹ️ 系统基本信息（OS、架构、核心数、运行时间）

**操作说明：**
- 按 `Q`、`Esc` 或 `Enter` 返回主菜单
- 自动刷新，无需手动操作

**注意事项：**
- ⚠️ 温度监控需要管理员权限
- 💡 建议以管理员身份运行以获得完整功能
- 📦 详细说明请参考 [REALTIME_MONITOR.md](REALTIME_MONITOR.md)

### 📟 CLI 模式（命令行）

#### 修复图标缓存
```powershell
lemo fix-icon-cache
```

选项：
- `-r, --restart-explorer <true|false>` - 是否自动重启资源管理器（默认: true）

#### 清理临时文件
```powershell
# 仅清理系统临时文件
lemo clean-temp

# 清理系统+用户临时文件（推荐）
lemo clean-temp --include-user
# 或简写
lemo clean-temp -i
```

**清理内容：**

**基础清理**（无需参数）:
- ✅ Windows 系统临时文件 (`C:\Windows\Temp`)
- ✅ Windows 预读缓存 (`C:\Windows\Prefetch`)
- ✅ Windows 回收站
- ✅ **深度扫描系统驱动器**（智能递归，限制深度）：
  - `*.tmp` - 临时文件
  - `*.log` - 日志文件
  - `*.gid` - Windows 帮助索引
  - `*.chk` - 磁盘检查碎片
  - `*.old` - 旧文件备份
  - `*.bak` - 备份文件
  - `*._mp` - 临时媒体文件

**扩展清理**（使用 `-i` 参数）:
- ✅ 用户临时文件夹 (`%TEMP%`)
- ✅ 用户 Cookies (`%USERPROFILE%\Cookies`)
- ✅ 最近使用文件 (`%USERPROFILE%\Recent`)
- ✅ IE 临时文件 (`%USERPROFILE%\Local Settings\Temporary Internet Files`)
- ✅ 本地临时文件 (`%USERPROFILE%\Local Settings\Temp`)

**智能优化**：
- ⚡ 限制递归深度（最多3层），避免过深搜索
- 🛡️ 自动跳过系统关键目录（Windows, Program Files, ProgramData 等）
- 📊 实时进度反馈（每 100 个文件显示进度）
- 🚀 执行时间：30秒 - 3分钟（取决于系统文件数量）

**功能特性：**
- ✅ 深度清理系统临时文件（智能递归搜索）
- ✅ 实时进度反馈（每 100 个文件显示进度）
- ✅ 详细的删除日志（显示前5个删除的文件）
- ✅ 跳过日志（显示被占用或无权限的文件）
- ✅ 统计释放的磁盘空间 (MB/GB)
- ✅ 总计删除/跳过的文件数量
- ✅ 智能深度限制，平衡性能和效果

#### 安装到系统
```powershell
lemo install
```

**功能说明：**
- ✅ 自动复制可执行文件到 `%LOCALAPPDATA%\lemo`
- ✅ 自动添加到系统 PATH（需要管理员权限）
- ✅ 安装后可在任何位置直接运行 `lemo` 命令
- ✅ 重启终端后生效

#### 从系统卸载
```powershell
lemo uninstall
```

**功能说明：**
- ✅ 从系统 PATH 中移除
- ✅ 删除安装目录
- ✅ 完全清理（需要管理员权限）

#### 查看帮助
```powershell
lemo --help
lemo <command> --help
```

## 安装

### 方法 1: 使用内置安装命令（推荐）

1. 首先编译 Release 版本：
```powershell
cargo build --release
```

2. 运行安装命令：
```powershell
# CLI 方式
.\target\release\lemo.exe install

# 或使用 TUI 方式（进入后选择 "Install to System"）
.\target\release\lemo.exe
```

安装后，程序会自动添加到系统 PATH，你可以在任何位置直接运行 `lemo` 命令。

### 方法 2: 手动安装

1. 编译项目：
```powershell
cargo build --release
```

2. 将 `target\release\lemo.exe` 复制到你想要的位置（如 `C:\Program Files\lemo\`）

3. 将该目录添加到系统 PATH：
   - 右键"此电脑" → "属性" → "高级系统设置"
   - 点击"环境变量"
   - 在"系统变量"中找到 `Path`，点击"编辑"
   - 添加 lemo.exe 所在目录的路径

## 卸载

### 使用内置卸载命令（推荐）

```powershell
# CLI 方式
lemo uninstall

# 或使用 TUI 方式（进入后选择 "Uninstall from System"）
lemo
```

## 开发

### 构建
```powershell
cargo build
```

### 运行
```powershell
cargo run -- <command>
```

### 编译 Release 版本
```powershell
cargo build --release
```

## 系统要求

- Windows 10/11
- 某些功能需要管理员权限

## 依赖

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [ratatui](https://github.com/ratatui-org/ratatui) - 终端 UI 框架
- [crossterm](https://github.com/crossterm-rs/crossterm) - 跨平台终端控制
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - 系统信息获取
- [winapi](https://github.com/retep998/winapi-rs) - Windows API 调用
- [anyhow](https://github.com/dtolnay/anyhow) - 错误处理
- [chrono](https://github.com/chronotope/chrono) - 日期时间处理
