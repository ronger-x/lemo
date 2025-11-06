# 应用发布者信息维护指南

## 📋 概述

本指南说明如何维护 Lemo 应用的发布者信息，这些信息会显示在 Windows 系统的各个位置，影响用户对应用的信任度和专业性。

## 🎯 发布者信息的重要性

正确配置发布者信息可以：
- ✅ 提升应用的专业性和可信度
- ✅ 在任务管理器中显示友好的应用名称
- ✅ 在文件属性中展示完整的版本信息
- ✅ 符合 Windows 应用商店/企业部署的规范要求
- ✅ 便于用户识别和管理应用程序

## 📝 核心信息字段详解

### 1. **CompanyName** - 公司/发布者名称 ⭐

```rc
VALUE "CompanyName", "ronger.io"
```

**显示位置：**
- 文件属性 → 详细信息 → 公司
- Windows 安装程序中的发布者名称
- 数字签名中的发布者（如果应用已签名）
- 某些安全提示对话框

**建议填写：**
- 个人开发者：使用你的名字或品牌名，如 "ronger.io"、"Zhang San"
- 组织/公司：使用正式的公司名称，如 "Microsoft Corporation"
- 开源项目：使用项目组织名，如 "Lemo Contributors"

**示例：**
```rc
VALUE "CompanyName", "ronger.io"              // 个人品牌
VALUE "CompanyName", "ACME Corporation"       // 公司
VALUE "CompanyName", "Open Source Community"  // 开源社区
```

---

### 2. **FileDescription** - 文件描述 ⭐⭐⭐

```rc
VALUE "FileDescription", "Lemo - Windows System Toolkit"
```

**显示位置：**
- 任务管理器中的"名称"列 ⭐ **最重要**
- 文件属性 → 详细信息 → 文件描述
- Windows 资源管理器的详细视图

**建议填写：**
- 简洁明了，建议 30 字符以内
- 包含应用主要功能
- 使用英文或中文（根据目标用户）

**示例：**
```rc
VALUE "FileDescription", "Lemo - Windows System Toolkit"
VALUE "FileDescription", "Lemo 系统工具集"
VALUE "FileDescription", "Windows 维护工具"
```

---

### 3. **ProductName** - 产品名称 ⭐⭐

```rc
VALUE "ProductName", "Lemo"
```

**显示位置：**
- 文件属性 → 详细信息 → 产品名称
- 控制面板 → 程序和功能（如果安装）
- Windows 事件日志

**建议填写：**
- 产品的官方名称
- 保持简洁，通常是应用的品牌名

**示例：**
```rc
VALUE "ProductName", "Lemo"
VALUE "ProductName", "Lemo Toolkit"
VALUE "ProductName", "Lemo 系统工具"
```

---

### 4. **LegalCopyright** - 版权信息 ⭐

```rc
VALUE "LegalCopyright", "Copyright (C) 2025"
```

**显示位置：**
- 文件属性 → 详细信息 → 版权
- 关于对话框（如果应用有）

**建议填写：**
- 标准格式：`Copyright (C) [年份] [持有者]`
- 包含版权年份和持有者名称
- 可以添加许可证信息

**示例：**
```rc
VALUE "LegalCopyright", "Copyright (C) 2025 ronger.io"
VALUE "LegalCopyright", "Copyright (C) 2025 ronger.io. All rights reserved."
VALUE "LegalCopyright", "Copyright (C) 2025 ronger.io. Licensed under MIT."
VALUE "LegalCopyright", "(C) 2024-2025 Lemo Contributors"
```

---

### 5. **FileVersion** 和 **ProductVersion** - 版本号 ⭐⭐

```rc
VALUE "FileVersion",    "0.2.3.0"
VALUE "ProductVersion", "0.2.3.0"
```

**显示位置：**
- 文件属性 → 详细信息 → 文件版本/产品版本
- 程序和功能列表中的版本
- 自动更新检查

**版本号格式：**
- 标准格式：`主版本.次版本.修订号.构建号`
- 示例：`1.2.3.4`
  - `1` - 主版本（Major）：重大功能变更
  - `2` - 次版本（Minor）：新增功能
  - `3` - 修订号（Patch）：Bug 修复
  - `4` - 构建号（Build）：构建编号（可选）

**同步要求：**
- `FILEVERSION` 和 `VALUE "FileVersion"` 必须一致
- `PRODUCTVERSION` 和 `VALUE "ProductVersion"` 必须一致
- 通常 FileVersion 和 ProductVersion 保持相同

**示例：**
```rc
// 开发版本
FILEVERSION     0,2,3,0
VALUE "FileVersion",    "0.2.3.0"

// 正式版本
FILEVERSION     1,0,0,0
VALUE "FileVersion",    "1.0.0.0"

// 包含构建号
FILEVERSION     1,2,3,4567
VALUE "FileVersion",    "1.2.3.4567"
```

---

### 6. **InternalName** - 内部名称

```rc
VALUE "InternalName", "lemo"
```

**用途：**
- 内部标识符，通常是可执行文件的基本名称
- 供开发者和系统内部使用

**建议填写：**
- 使用小写字母
- 通常与可执行文件名相同（不含扩展名）

---

### 7. **OriginalFilename** - 原始文件名

```rc
VALUE "OriginalFilename", "lemo.exe"
```

**用途：**
- 记录文件的原始名称
- 防止文件被重命名后无法识别
- Windows 系统完整性检查

**建议填写：**
- 必须包含扩展名（.exe）
- 与实际编译生成的文件名一致

---

## 🔄 版本号维护策略

### 方法 1: 手动维护（当前方式）

每次发布新版本时，需要同时更新 3 个地方：

1. **resources.rc** - 更新 FILEVERSION 和所有版本字符串
2. **Cargo.toml** - 更新 package.version
3. **src/main.rs** - 更新 #[command(version = "...")] （如果有）

### 方法 2: 自动化同步（推荐）

可以使用构建脚本自动从 `Cargo.toml` 读取版本号：

#### 步骤 1: 修改 build.rs

```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 读取版本号
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let parts: Vec<&str> = version.split('.').collect();
    
    // 生成动态 resources.rc
    let template = fs::read_to_string("resources.rc.template").unwrap();
    let content = template
        .replace("{{VERSION}}", &version)
        .replace("{{VERSION_COMMA}}", &format!("{},{},{},0", 
            parts.get(0).unwrap_or(&"0"),
            parts.get(1).unwrap_or(&"0"),
            parts.get(2).unwrap_or(&"0")
        ));
    
    fs::write("resources.rc", content).unwrap();
    
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}
```

#### 步骤 2: 创建 resources.rc.template

```rc
VALUE "FileVersion",    "{{VERSION}}.0"
VALUE "ProductVersion", "{{VERSION}}.0"
FILEVERSION     {{VERSION_COMMA}}
PRODUCTVERSION  {{VERSION_COMMA}}
```

这样只需要更新 `Cargo.toml` 中的版本号，其他地方会自动同步。

---

## 📊 完整配置示例

### 个人开发者配置

```rc
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName",      "ronger.io"
            VALUE "FileDescription",  "Lemo - Windows System Toolkit"
            VALUE "FileVersion",      "1.0.0.0"
            VALUE "InternalName",     "lemo"
            VALUE "LegalCopyright",   "Copyright (C) 2025 ronger.io. Licensed under MIT."
            VALUE "OriginalFilename", "lemo.exe"
            VALUE "ProductName",      "Lemo"
            VALUE "ProductVersion",   "1.0.0.0"
        END
    END
END
```

### 公司/组织配置

```rc
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName",      "ACME Corporation"
            VALUE "FileDescription",  "ACME System Management Tool"
            VALUE "FileVersion",      "2.1.5.0"
            VALUE "InternalName",     "acme-tool"
            VALUE "LegalCopyright",   "Copyright (C) 2024-2025 ACME Corporation. All rights reserved."
            VALUE "OriginalFilename", "acme-tool.exe"
            VALUE "ProductName",      "ACME System Tool"
            VALUE "ProductVersion",   "2.1.5.0"
        END
    END
END
```

### 开源项目配置

```rc
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName",      "Lemo Contributors"
            VALUE "FileDescription",  "Lemo - Open Source Windows Toolkit"
            VALUE "FileVersion",      "0.9.2.0"
            VALUE "InternalName",     "lemo"
            VALUE "LegalCopyright",   "Copyright (C) 2025 Lemo Contributors. Licensed under MIT License."
            VALUE "OriginalFilename", "lemo.exe"
            VALUE "ProductName",      "Lemo"
            VALUE "ProductVersion",   "0.9.2.0"
        END
    END
END
```

---

## 🔍 验证发布者信息

### 方法 1: 通过文件属性查看

1. 右键点击编译后的 `lemo.exe`
2. 选择 "属性"
3. 切换到 "详细信息" 标签
4. 检查所有字段是否正确

### 方法 2: 通过任务管理器查看

1. 运行 `lemo.exe`
2. 打开任务管理器（Ctrl + Shift + Esc）
3. 切换到 "详细信息" 标签
4. 查看 "描述" 列（显示 FileDescription）

### 方法 3: 使用 PowerShell 查看

```powershell
# 查看所有版本信息
(Get-Item .\target\release\lemo.exe).VersionInfo | Format-List

# 查看特定字段
(Get-Item .\target\release\lemo.exe).VersionInfo.CompanyName
(Get-Item .\target\release\lemo.exe).VersionInfo.FileDescription
(Get-Item .\target\release\lemo.exe).VersionInfo.ProductVersion
```

---

## 📝 维护清单

每次发布新版本时的检查清单：

- [ ] 更新 `Cargo.toml` 中的 version
- [ ] 更新 `resources.rc` 中的 FILEVERSION
- [ ] 更新 `resources.rc` 中的 PRODUCTVERSION
- [ ] 更新 `resources.rc` 中的 FileVersion 字符串
- [ ] 更新 `resources.rc` 中的 ProductVersion 字符串
- [ ] 检查 LegalCopyright 年份是否需要更新
- [ ] 编译：`cargo build --release`
- [ ] 验证版本信息：右键 lemo.exe → 属性 → 详细信息
- [ ] 测试运行并检查任务管理器中的显示

---

## ⚠️ 常见问题

### Q: 为什么版本号要写两次？

A: Windows 资源文件要求：
- `FILEVERSION 0,2,3,0` - 二进制格式（逗号分隔）
- `VALUE "FileVersion", "0.2.3.0"` - 字符串格式（点分隔）

两者必须对应，否则可能导致显示不一致。

### Q: CompanyName 应该填什么？

A: 
- **个人开发者**：填写你的名字、昵称或个人品牌
- **公司**：填写公司注册名称
- **开源项目**：可以填 "Contributors"、"Community" 或项目名称

### Q: 如何处理中文信息？

A: resources.rc 支持 Unicode，可以直接使用中文：

```rc
VALUE "CompanyName",      "荣格科技"
VALUE "FileDescription",  "Lemo 系统维护工具"
VALUE "ProductName",      "Lemo 工具集"
```

但建议在国际化应用中使用英文，或提供多语言资源块。

### Q: 版本号必须是 4 位吗？

A: 不是必须的，但建议使用 4 位格式（X.Y.Z.B）：
- 前 3 位用于语义化版本（主.次.修订）
- 第 4 位用于构建编号（可选，通常填 0）

### Q: 如何添加多语言支持？

A: 可以添加多个 BLOCK，每个对应一种语言：

```rc
BLOCK "StringFileInfo"
BEGIN
    // 英文 (0x0409 = en-US)
    BLOCK "040904b0"
    BEGIN
        VALUE "ProductName", "Lemo"
    END
    
    // 中文 (0x0804 = zh-CN)
    BLOCK "080404b0"
    BEGIN
        VALUE "ProductName", "Lemo 工具集"
    END
END
```

---

## 🎯 推荐配置

对于 Lemo 项目，推荐以下配置：

```rc
VALUE "CompanyName",      "ronger.io"
VALUE "FileDescription",  "Lemo - Windows System Toolkit"
VALUE "FileVersion",      "从 Cargo.toml 同步"
VALUE "InternalName",     "lemo"
VALUE "LegalCopyright",   "Copyright (C) 2025 ronger.io. Licensed under MIT."
VALUE "OriginalFilename", "lemo.exe"
VALUE "ProductName",      "Lemo"
VALUE "ProductVersion",   "从 Cargo.toml 同步"
```

这样配置可以：
- ✅ 清晰标识开发者身份
- ✅ 在任务管理器中友好显示
- ✅ 提供版权和许可证信息
- ✅ 保持版本号同步

---

## 📚 参考资料

- [Microsoft Docs - VERSIONINFO Resource](https://learn.microsoft.com/en-us/windows/win32/menurc/versioninfo-resource)
- [embed-resource crate](https://crates.io/crates/embed-resource)
- [Semantic Versioning 2.0.0](https://semver.org/)
