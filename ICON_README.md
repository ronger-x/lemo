# 图标文件说明

## 📍 放置位置

请将你的图标文件命名为 `lemo.ico` 并放置在此目录（项目根目录）：

```
lemo/
├── lemo.ico          ← 将图标文件放在这里
├── resources.rc
├── build.rs
├── Cargo.toml
└── src/
```

## 🎨 如何获取图标

### 方法 1: 使用在线工具生成

#### 推荐工具：
1. **Favicon.io** - https://favicon.io/emoji-favicons/
   - 可以从 Emoji 生成图标
   - 例如：🍋 柠檬、🔧 扳手、⚙️ 齿轮

2. **ICO Convert** - https://www.icoconverter.com/
   - 可以从 PNG/JPG 转换为 ICO
   - 支持多种尺寸

3. **Convertio** - https://convertio.co/zh/png-ico/
   - 在线格式转换

### 方法 2: 使用现有图标

可以从以下来源获取免费图标：
- https://icon-icons.com/
- https://www.flaticon.com/
- https://icons8.com/

### 方法 3: 使用系统图标提取工具

Windows 自带很多图标，可以使用工具提取：
- IcoFX (https://icofx.ro/)
- Resource Hacker (http://www.angusj.com/resourcehacker/)

## 📐 图标规格要求

标准的 ICO 文件应包含以下尺寸：
- 16x16 像素（小图标、标题栏）
- 32x32 像素（任务栏、快捷方式）
- 48x48 像素（资源管理器中等图标）
- 256x256 像素（资源管理器大图标、高 DPI）

## ⚡ 快速开始示例

### 使用柠檬 Emoji 图标（与应用主题匹配）

1. 访问：https://favicon.io/emoji-favicons/lemon/
2. 点击 "Download" 下载图标包
3. 解压后找到 `favicon.ico` 文件
4. 将其重命名为 `lemo.ico`
5. 放到项目根目录（与 Cargo.toml 同级）
6. 运行编译：`cargo build --release`

### 使用工具主题图标

如果想用工具/扳手主题：
1. 访问：https://favicon.io/emoji-favicons/wrench/
2. 下载并按上述步骤操作

## 🔍 验证图标

放置好 `lemo.ico` 后：

1. 检查文件是否存在：
   ```powershell
   Test-Path .\lemo.ico
   ```

2. 编译项目：
   ```powershell
   cargo build --release
   ```

3. 检查编译输出中是否有资源嵌入相关信息

4. 查看生成的 EXE 图标：
   - 打开 `target\release\` 目录
   - 查看 `lemo.exe` 的图标

## ❌ 如果暂时没有图标

如果你还没有准备图标文件，可以先注释掉资源引用：

编辑 `resources.rc` 文件，注释掉图标行：

```rc
// 暂时注释掉图标
// 1 ICON "lemo.ico"

// 版本信息仍然会被嵌入
1 VERSIONINFO
...
```

这样编译时仍会嵌入版本信息，只是不会有自定义图标。

## 📝 注意事项

- ICO 文件必须是标准的 Windows ICO 格式
- 文件名必须是 `lemo.ico`（或修改 resources.rc 中的引用）
- 图标文件大小建议不超过 100KB
- 确保包含多种尺寸以获得最佳显示效果
