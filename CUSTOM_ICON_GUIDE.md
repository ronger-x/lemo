# 自定义应用图标和标题指南

## 📋 概述

本指南说明如何为 Lemo 应用程序自定义 Windows 任务栏图标和窗口标题栏。

## 🎨 修改应用图标

### 1. 准备图标文件

需要一个 `.ico` 格式的图标文件，建议包含多种尺寸：
- 16x16 像素（小图标）
- 32x32 像素（任务栏）
- 48x48 像素（资源管理器）
- 256x256 像素（高分辨率）

### 2. 放置图标文件

将图标文件命名为 `lemo.ico`，放在项目根目录：

```
lemo/
├── lemo.ico          ← 放在这里
├── resources.rc
├── build.rs
├── Cargo.toml
└── src/
```

### 3. 在线图标制作工具

如果没有 .ico 文件，可以使用以下工具：
- https://www.icoconverter.com/ - 在线 ICO 转换器
- https://favicon.io/ - Favicon 生成器
- https://convertio.co/zh/png-ico/ - 格式转换

或使用 Emoji 转图标：
- https://favicon.io/emoji-favicons/lemon/ - Emoji 转 ICO

## 🏷️ 修改窗口标题和版本信息

编辑 `resources.rc` 文件中的信息：

```rc
1 VERSIONINFO
FILEVERSION     0,2,3,0
PRODUCTVERSION  0,2,3,0
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName",      "你的公司名"
            VALUE "FileDescription",  "你的应用描述"
            VALUE "FileVersion",      "0.2.3.0"
            VALUE "InternalName",     "应用内部名称"
            VALUE "LegalCopyright",   "版权信息"
            VALUE "OriginalFilename", "lemo.exe"
            VALUE "ProductName",      "产品名称"
            VALUE "ProductVersion",   "0.2.3.0"
        END
    END
END
```

### 关键字段说明：

- **FileDescription**: 在任务管理器中显示的描述
- **ProductName**: 产品名称（右键属性可见）
- **CompanyName**: 公司/开发者名称
- **LegalCopyright**: 版权信息
- **FileVersion**: 文件版本号

## 🔧 修改控制台窗口标题

如果想在运行时动态修改控制台标题，可以在 `src/main.rs` 中添加：

```rust
use std::io::Write;

fn main() -> Result<()> {
    // 设置控制台窗口标题
    set_console_title("🍋 Lemo - Windows System Toolkit");
    
    // ... 其余代码
}

#[cfg(windows)]
fn set_console_title(title: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::wincon::SetConsoleTitleW;
    
    let wide: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        SetConsoleTitleW(wide.as_ptr());
    }
}

#[cfg(not(windows))]
fn set_console_title(_title: &str) {
    // 非 Windows 平台不执行
}
```

## 🏗️ 编译应用

完成上述修改后，重新编译：

```powershell
# 调试版本
cargo build

# 发布版本（推荐）
cargo build --release
```

编译后的 EXE 文件将包含：
- ✅ 自定义图标（任务栏、桌面快捷方式）
- ✅ 版本信息（右键属性可见）
- ✅ 文件描述（任务管理器中显示）

## 📝 验证结果

编译完成后验证：

1. **查看图标**：
   - 在资源管理器中查看 `lemo.exe`
   - 创建桌面快捷方式查看图标
   - 运行时查看任务栏图标

2. **查看版本信息**：
   - 右键 `lemo.exe` → 属性 → 详细信息
   - 检查版本号、描述、公司等信息

3. **查看任务管理器**：
   - 运行程序后打开任务管理器
   - 查看"FileDescription"是否正确显示

## 🎯 快速示例

### 使用柠檬 Emoji 图标

1. 访问 https://favicon.io/emoji-favicons/lemon/
2. 下载生成的 `favicon.ico`
3. 重命名为 `lemo.ico` 并放到项目根目录
4. 运行 `cargo build --release`
5. 完成！

### 自定义为工具箱主题

修改 `resources.rc`：

```rc
VALUE "FileDescription",  "Windows System Maintenance Toolkit"
VALUE "ProductName",      "Lemo Toolkit"
VALUE "CompanyName",      "YourName"
```

## ⚠️ 常见问题

### Q: 编译后图标没有变化？
A: 确保 `lemo.ico` 文件存在于项目根目录，并且重新编译了完整的 release 版本。

### Q: 图标显示模糊？
A: 确保 .ico 文件包含多种尺寸（16x16, 32x32, 48x48, 256x256）。

### Q: 如何移除图标？
A: 注释或删除 `resources.rc` 中的 `1 ICON "lemo.ico"` 行。

## 📚 更多资源

- [Windows Resource Files 文档](https://learn.microsoft.com/en-us/windows/win32/menurc/resource-files)
- [embed-resource crate](https://crates.io/crates/embed-resource)
- [ICO 格式规范](https://en.wikipedia.org/wiki/ICO_(file_format))
