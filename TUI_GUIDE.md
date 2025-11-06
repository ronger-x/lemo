# Lemo TUI (Terminal User Interface) Version

## 🎉 New Features

### Interactive TUI Mode
Lemo now features a beautiful Terminal User Interface built with `ratatui`!

## 🚀 Usage

### TUI Mode (Default)
Simply run without arguments to enter interactive mode:
```powershell
lemo
```

This will launch a fullscreen TUI with:
- 🍋 Beautiful header with branding
- 📋 Interactive menu with keyboard navigation
- ⌨️ Intuitive keyboard controls
- 🎨 Color-coded interface

### CLI Mode (Still Available)
All original CLI commands still work:
```powershell
# Fix icon cache
lemo fix-icon-cache

# Clean temporary files
lemo clean-temp
lemo clean-temp --include-user

# Show system information
lemo sys-info
```

## ⌨️ TUI Controls

- **Arrow Keys** or **j/k**: Navigate up/down
- **Enter** or **Space**: Execute selected action  
- **q** or **Esc**: Quit

## 📋 TUI Menu Options

1. **🔧 Fix Icon Cache** - Fix Windows icon cache issues
2. **🧹 Clean Temp Files** - Clean system temporary files
3. **💻 System Info** - Display detailed system information
4. **🚪 Exit** - Quit the application

## 🎨 Features

### Visual Design
- Clean, modern terminal interface
- Color-coded elements for better readability
- Highlighted selection with yellow background
- Bordered sections for organization

### User Experience
- Smooth keyboard navigation
- Clear instructions in footer
- Automatic admin privilege elevation
- Seamless switch between TUI and execution output

### Functionality Preservation
- All original features work exactly the same
- Both TUI and CLI modes available
- No loss of functionality

## 🏗️ Architecture

### Code Structure
```
src/
├── main.rs    - TUI interface and application logic
└── utils.rs   - Core functionality (shared between TUI and CLI)
```

### Dependencies
- **ratatui**: TUI framework
- **crossterm**: Cross-platform terminal manipulation
- **anyhow**: Error handling
- **clap**: CLI argument parsing
- **sysinfo**: System information gathering
- **winapi**: Windows-specific APIs

## 💡 Benefits of TUI

1. **Better UX**: More intuitive than remembering CLI commands
2. **Visual Feedback**: See available options at a glance
3. **Safer**: Confirm actions before execution
4. **Modern**: Contemporary terminal application design
5. **Efficient**: Quick navigation with keyboard shortcuts

## 🔄 Workflow Example

1. Run `lemo` (with admin privileges)
2. See the main menu with all options
3. Navigate with arrow keys
4. Press Enter to execute
5. View output in terminal
6. Press Enter to return to TUI
7. Select another option or quit

## 📦 Installation

Same as before - use the install script:
```powershell
powershell -ExecutionPolicy Bypass -File install.ps1
```

Then you can run `lemo` from anywhere!

## 🎯 Best Use Cases

### Use TUI Mode When:
- Exploring available features
- Unsure which command to run
- Want visual confirmation
- Running multiple operations

### Use CLI Mode When:
- Automating with scripts
- Know exact command needed
- Running from batch files
- Remote execution

## 🚧 Technical Details

- **TUI Framework**: ratatui 0.29
- **Terminal**: crossterm 0.28
- **Error Handling**: anyhow 1.0
- **Encoding**: UTF-8 throughout

## 🎨 Screenshots

```
╔══════════════════════════════════════════════╗
║   🍋 Lemo - Windows System Toolkit           ║
╚══════════════════════════════════════════════╝
┌─ Main Menu ──────────────────────────────────┐
│ 🔧 Fix Icon Cache                            │
│ 🧹 Clean Temp Files                          │
│ 💻 System Info                               │
│ 🚪 Exit                                      │
└──────────────────────────────────────────────┘
╔══════════════════════════════════════════════╗
║ ↑/↓: Navigate | Enter: Execute | Q/Esc: Quit ║
╚══════════════════════════════════════════════╝
```

## 🔮 Future Enhancements

Potential future additions:
- Progress bars for long operations
- Real-time log streaming in TUI
- More detailed system monitoring
- Configuration options
- Help screens

## ⚡ Performance

- Instant startup
- Minimal resource usage
- Responsive keyboard input
- No lag or delays
