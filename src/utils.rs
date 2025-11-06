// Utility functions module
use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winuser::SW_SHOWNORMAL;

// Check if running as administrator
pub fn is_admin() -> bool {
    use winapi::um::processthreadsapi::*;
    use winapi::um::securitybaseapi::*;
    use winapi::um::winnt::*;

    unsafe {
        let mut token_handle = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        winapi::um::handleapi::CloseHandle(token_handle);

        result != 0 && elevation.TokenIsElevated != 0
    }
}

// Run as administrator
pub fn run_as_admin() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_path_str = exe_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Cannot get executable path"))?;

    let args: Vec<String> = env::args().skip(1).collect();
    let params = args.join(" ");

    let operation: Vec<u16> = "runas\0".encode_utf16().collect();
    let file: Vec<u16> = format!("{}\0", exe_path_str).encode_utf16().collect();
    let parameters: Vec<u16> = format!("{}\0", params).encode_utf16().collect();

    unsafe {
        let result = ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters.as_ptr(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        );

        if result as i32 <= 32 {
            return Err(anyhow::anyhow!("Cannot elevate privileges"));
        }
    }

    Ok(())
}

// Fix icon cache
pub fn fix_icon_cache(restart_explorer: bool) -> Result<()> {
    println!("🔧 Fixing icon cache...");

    println!("⏳ Closing Windows Explorer...");
    let _ = Command::new("taskkill")
        .args(&["/f", "/im", "explorer.exe"])
        .output();

    thread::sleep(Duration::from_secs(2));

    let user_profile = env::var("USERPROFILE")?;
    let mut cache_files = Vec::new();

    let icon_cache = PathBuf::from(&user_profile).join(r"AppData\Local\IconCache.db");
    cache_files.push(icon_cache);

    let icon_cache_pattern =
        PathBuf::from(&user_profile).join(r"AppData\Local\Microsoft\Windows\Explorer");

    if let Ok(entries) = fs::read_dir(icon_cache_pattern) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(name) = file_name.to_str() {
                    if name.starts_with("iconcache_") && name.ends_with(".db") {
                        cache_files.push(path);
                    }
                }
            }
        }
    }

    let mut deleted_count = 0;
    let mut skipped_count = 0;

    for file in cache_files {
        match fs::remove_file(&file) {
            Ok(_) => {
                println!("✅ Deleted: {:?}", file);
                deleted_count += 1;
            }
            Err(e) => {
                println!("⚠️  Skipped: {:?} ({})", file, e);
                skipped_count += 1;
            }
        }
    }

    println!(
        "\n📊 Summary: Deleted {} files, Skipped {} files",
        deleted_count, skipped_count
    );

    if restart_explorer {
        println!("🔄 Restarting Windows Explorer...");
        Command::new("explorer.exe").spawn()?;
        println!("✨ Fix completed! Desktop will restore in a few seconds.");
        thread::sleep(Duration::from_secs(3));
    } else {
        println!("✨ Fix completed! Please restart Explorer manually.");
    }

    Ok(())
}

// Output trait for different output methods
trait CleanOutput {
    fn print(&mut self, msg: &str);
    fn print_empty(&mut self) {
        self.print("");
    }
}

// Console output
struct ConsoleOutput;
impl CleanOutput for ConsoleOutput {
    fn print(&mut self, msg: &str) {
        println!("{}", msg);
    }
}

// Callback output
struct CallbackOutput<'a> {
    callback: &'a mut Box<dyn FnMut(String) + Send>,
}
impl<'a> CleanOutput for CallbackOutput<'a> {
    fn print(&mut self, msg: &str) {
        (self.callback)(msg.to_string());
    }
}

// 子函数：清理 Windows Temp 目录
fn clean_windows_temp<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let windows_temp = PathBuf::from(r"C:\Windows\Temp");
    if !windows_temp.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning Windows temp directory: {}", windows_temp.display()));
    let (deleted, failed, size) = clean_directory_with_output(&windows_temp, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理 Windows Prefetch
fn clean_windows_prefetch<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let prefetch = PathBuf::from(r"C:\Windows\Prefetch");
    if !prefetch.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning Windows prefetch: {}", prefetch.display()));
    let (deleted, failed, size) = clean_directory_with_output(&prefetch, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理 Windows 目录下的 .bak 文件
fn clean_windows_bak_files<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let windir = match env::var("windir") {
        Ok(dir) => dir,
        Err(_) => return Ok((0, 0, 0)),
    };

    let windows_dir = PathBuf::from(&windir);
    output.print_empty();
    output.print(&format!("📁 Cleaning Windows directory backup files: {}", windows_dir.display()));
    
    let bak_extensions = vec!["bak"];
    let (deleted, failed, size) = clean_files_by_extension_with_progress(
        &windows_dir,
        &bak_extensions,
        &mut |_, _, _, _| {},
        0,
    )?;
    
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理回收站
fn clean_recycle_bin<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    use winapi::um::shellapi::SHEmptyRecycleBinW;
    
    output.print_empty();
    output.print("🗑️  Emptying recycle bin...");
    
    // 使用 Windows API 清空回收站
    // SHERB_NOCONFIRMATION = 0x00000001 (不显示确认对话框)
    // SHERB_NOPROGRESSUI = 0x00000002 (不显示进度对话框)
    // SHERB_NOSOUND = 0x00000004 (不播放声音)
    let flags = 0x00000001 | 0x00000002 | 0x00000004;
    
    unsafe {
        // NULL 表示清空所有驱动器的回收站
        let result = SHEmptyRecycleBinW(
            std::ptr::null_mut(),
            std::ptr::null(),
            flags
        );
        
        if result == 0 {
            output.print("   ✅ Recycle bin emptied successfully");
            // 注意：无法准确获取删除的文件数量和大小，返回估计值
            Ok((1, 0, 0))
        } else {
            output.print(&format!("   ⚠️  Failed to empty recycle bin (error code: 0x{:X})", result));
            Ok((0, 1, 0))
        }
    }
}

// 子函数：清理系统驱动器临时文件
fn clean_system_drive_temp_files<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    output.print_empty();
    output.print("📁 Scanning system drive for temp files (this may take a while)...");
    
    let system_drive = env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
    let extensions = vec!["tmp", "log", "gid", "chk", "old", "bak", "_mp"];
    
    let (deleted, failed, size) = clean_files_by_extension_with_progress(
        &PathBuf::from(&system_drive),
        &extensions,
        &mut |_, _, _, _| {},
        0,
    )?;
    
    output.print(&format!(
        "   ✅ Completed: {} items deleted, {} skipped, {:.2} MB freed",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理用户临时目录
fn clean_user_temp<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let temp = match env::var("TEMP") {
        Ok(t) => t,
        Err(_) => return Ok((0, 0, 0)),
    };

    let user_temp = PathBuf::from(temp);
    if !user_temp.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning user temp directory: {}", user_temp.display()));
    let (deleted, failed, size) = clean_directory_with_output(&user_temp, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理用户 Cookies
fn clean_user_cookies<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let userprofile = match env::var("USERPROFILE") {
        Ok(p) => p,
        Err(_) => return Ok((0, 0, 0)),
    };

    let cookies = PathBuf::from(&userprofile).join("Cookies");
    if !cookies.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning user cookies: {}", cookies.display()));
    let (deleted, failed, size) = clean_directory_with_output(&cookies, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理用户最近文件
fn clean_user_recent<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let userprofile = match env::var("USERPROFILE") {
        Ok(p) => p,
        Err(_) => return Ok((0, 0, 0)),
    };

    let recent = PathBuf::from(&userprofile).join("Recent");
    if !recent.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning user recent files: {}", recent.display()));
    let (deleted, failed, size) = clean_directory_with_output(&recent, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理 IE 临时文件
fn clean_ie_temp_files<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let userprofile = match env::var("USERPROFILE") {
        Ok(p) => p,
        Err(_) => return Ok((0, 0, 0)),
    };

    let ie_temp = PathBuf::from(&userprofile).join(r"Local Settings\Temporary Internet Files");
    if !ie_temp.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning IE temporary files: {}", ie_temp.display()));
    let (deleted, failed, size) = clean_directory_with_output(&ie_temp, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// 子函数：清理用户本地临时文件
fn clean_user_local_temp<O: CleanOutput>(output: &mut O) -> Result<(usize, usize, u64)> {
    let userprofile = match env::var("USERPROFILE") {
        Ok(p) => p,
        Err(_) => return Ok((0, 0, 0)),
    };

    let local_temp = PathBuf::from(&userprofile).join(r"Local Settings\Temp");
    if !local_temp.exists() {
        return Ok((0, 0, 0));
    }

    output.print_empty();
    output.print(&format!("📁 Cleaning user local temp: {}", local_temp.display()));
    let (deleted, failed, size) = clean_directory_with_output(&local_temp, output)?;
    output.print(&format!(
        "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
        deleted, failed, size as f64 / 1024.0 / 1024.0
    ));
    
    Ok((deleted, failed, size))
}

// Core cleaning logic (shared by all variants)
fn clean_temp_core<O: CleanOutput>(include_user: bool, output: &mut O) -> Result<(usize, usize, u64)> {
    output.print("🧹 Cleaning temporary files...");
    output.print("═══════════════════════════════════════════════════");

    let mut total_deleted = 0;
    let mut total_failed = 0;
    let mut total_size_freed: u64 = 0;

    // 清理 Windows Temp 目录
    let (deleted, failed, size) = clean_windows_temp(output)?;
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;

    // 清理 Windows Prefetch
    let (deleted, failed, size) = clean_windows_prefetch(output)?;
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;

    // 清理 Windows 目录下的 .bak 文件
    let (deleted, failed, size) = clean_windows_bak_files(output)?;
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;

    // 清理回收站
    let (deleted, failed, size) = clean_recycle_bin(output)?;
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;

    // 清理系统驱动器临时文件
    let (deleted, failed, size) = clean_system_drive_temp_files(output)?;
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;

    // 如果需要清理用户相关目录
    if include_user {
        let (deleted, failed, size) = clean_user_temp(output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;

        let (deleted, failed, size) = clean_user_cookies(output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;

        let (deleted, failed, size) = clean_user_recent(output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;

        let (deleted, failed, size) = clean_ie_temp_files(output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;

        let (deleted, failed, size) = clean_user_local_temp(output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;
    }

    output.print_empty();
    output.print("═══════════════════════════════════════════════════");
    output.print("📊 Cleaning summary:");
    output.print(&format!("   Total deleted: {} items", total_deleted));
    output.print(&format!("   Total skipped: {} items", total_failed));
    output.print(&format!(
        "   Freed space: {:.2} MB ({:.2} GB)",
        total_size_freed as f64 / 1024.0 / 1024.0,
        total_size_freed as f64 / 1024.0 / 1024.0 / 1024.0
    ));
    output.print("═══════════════════════════════════════════════════");
    output.print("✨ Cleaning completed!");

    Ok((total_deleted, total_failed, total_size_freed))
}

// Clean temporary files (console output)
pub fn clean_temp(include_user: bool) -> Result<()> {
    let mut output = ConsoleOutput;
    clean_temp_core(include_user, &mut output)?;
    Ok(())
}

// Clean a directory with custom output (internal helper)
fn clean_directory_with_output<O: CleanOutput>(dir: &PathBuf, output: &mut O) -> Result<(usize, usize, u64)> {
    let mut deleted_count = 0;
    let mut failed_count = 0;
    let mut total_size = 0u64;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            let size = if path.is_file() {
                fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
            } else if path.is_dir() {
                calculate_dir_size(&path)
            } else {
                0
            };

            let result = if path.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };

            match result {
                Ok(_) => {
                    deleted_count += 1;
                    total_size += size;
                    if deleted_count <= 5 {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        let display_name = if filename.len() > 60 {
                            format!("{}...", &filename[..57])
                        } else {
                            filename.to_string()
                        };
                        output.print(&format!("   ✅ Deleted: {}", display_name));
                    }
                }
                Err(_) => {
                    failed_count += 1;
                    if failed_count <= 3 {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        let display_name = if filename.len() > 50 {
                            format!("{}...", &filename[..47])
                        } else {
                            filename.to_string()
                        };
                        output.print(&format!("   ⚠️  Skipped: {} (locked)", display_name));
                    }
                }
            }
        }

        if deleted_count > 5 {
            output.print(&format!("   ... and {} more items deleted", deleted_count - 5));
        }
        if failed_count > 3 {
            output.print(&format!("   ... and {} more items skipped", failed_count - 3));
        }
    }

    Ok((deleted_count, failed_count, total_size))
}

// Clean files by extension with progress callback
pub fn clean_files_by_extension_with_progress<F>(
    dir: &PathBuf,
    extensions: &[&str],
    progress_callback: &mut F,
    depth: usize,
) -> Result<(usize, usize, u64)>
where
    F: FnMut(&str, usize, usize, u64),
{
    let mut deleted_count = 0;
    let mut failed_count = 0;
    let mut total_size = 0u64;

    // 限制递归深度，避免过深（从第一级子目录开始计数）
    if depth > 5 {
        return Ok((0, 0, 0));
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // 只在根目录层级（depth == 0）跳过系统关键目录
            if depth == 0 {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        // 跳过核心系统目录
                        if name_str == "Windows" 
                            || name_str == "Program Files"
                            || name_str == "Program Files (x86)"
                            || name_str == "System Volume Information"
                            || name_str == "$Recycle.Bin"
                            || name_str == "ProgramData"
                            || name_str.starts_with('$')
                        {
                            continue;
                        }
                    }
                }
            }

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if extensions.contains(&ext_str) {
                            let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

                            match fs::remove_file(&path) {
                                Ok(_) => {
                                    deleted_count += 1;
                                    total_size += size;
                                    
                                    // 每删除一个文件就更新进度
                                    progress_callback(
                                        &path.display().to_string(),
                                        deleted_count,
                                        failed_count,
                                        total_size,
                                    );
                                }
                                Err(_) => {
                                    failed_count += 1;
                                }
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                // 递归清理子目录
                if let Ok((d, f, s)) = clean_files_by_extension_with_progress(
                    &path,
                    extensions,
                    progress_callback,
                    depth + 1,
                ) {
                    deleted_count += d;
                    failed_count += f;
                    total_size += s;
                }
            }
        }
    }

    Ok((deleted_count, failed_count, total_size))
}

// Calculate directory size
pub fn calculate_dir_size(dir: &PathBuf) -> u64 {
    let mut size = 0u64;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                size += fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            } else if path.is_dir() {
                size += calculate_dir_size(&path);
            }
        }
    }

    size
}

// Show system information
// ========================================
// Streaming output functions (callback-based)
// ========================================

// Fix icon cache with streaming output (callback-based)
pub fn fix_icon_cache_with_streaming() -> impl FnOnce(Box<dyn FnMut(String) + Send>) -> Result<()> + Send + 'static {
    |mut callback: Box<dyn FnMut(String) + Send>| {
        callback("🔧 Fixing icon cache...".to_string());
        callback(String::new());

        callback("⏳ Closing Windows Explorer...".to_string());
        let _ = Command::new("taskkill")
            .args(&["/f", "/im", "explorer.exe"])
            .output();

        thread::sleep(Duration::from_secs(2));

        let user_profile = match env::var("USERPROFILE") {
            Ok(p) => p,
            Err(_) => return Err(anyhow::anyhow!("Cannot get USERPROFILE")),
        };
        let mut cache_files = Vec::new();

        let icon_cache = PathBuf::from(&user_profile).join(r"AppData\Local\IconCache.db");
        cache_files.push(icon_cache);

        let icon_cache_pattern =
            PathBuf::from(&user_profile).join(r"AppData\Local\Microsoft\Windows\Explorer");

        if let Ok(entries) = fs::read_dir(icon_cache_pattern) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(name) = file_name.to_str() {
                        if name.starts_with("iconcache_") && name.ends_with(".db") {
                            cache_files.push(path);
                        }
                    }
                }
            }
        }

        let mut deleted_count = 0;
        let mut skipped_count = 0;

        for file in cache_files {
            match fs::remove_file(&file) {
                Ok(_) => {
                    callback(format!("✅ Deleted: {:?}", file));
                    deleted_count += 1;
                }
                Err(e) => {
                    callback(format!("⚠️  Skipped: {:?} ({})", file, e));
                    skipped_count += 1;
                }
            }
        }

        callback(String::new());
        callback(format!(
            "📊 Summary: Deleted {} files, Skipped {} files",
            deleted_count, skipped_count
        ));

        callback(String::new());
        callback("🔄 Restarting Windows Explorer...".to_string());
        match Command::new("explorer.exe").spawn() {
            Ok(_) => callback("✨ Fix completed! Desktop will restore in a few seconds.".to_string()),
            Err(e) => callback(format!("⚠️  Warning: Failed to restart Explorer: {}", e)),
        }
        thread::sleep(Duration::from_secs(3));

        Ok(())
    }
}

// Clean temp files with streaming output (callback-based)
pub fn clean_temp_with_streaming() -> impl FnOnce(Box<dyn FnMut(String) + Send>) -> Result<()> + Send + 'static {
    |mut callback: Box<dyn FnMut(String) + Send>| {
        let mut output = CallbackOutput { callback: &mut callback };
        clean_temp_core(false, &mut output)?;
        Ok(())
    }
}

// Clean directory with streaming output
// Show system info with streaming output (callback-based)
/// Install the application to the system
pub fn install_to_system() -> Result<()> {
    use std::os::windows::process::CommandExt;
    
    println!("Starting installation...");
    
    // Get installation path
    let local_appdata = env::var("LOCALAPPDATA")?;
    let install_path = PathBuf::from(local_appdata).join("lemo");
    
    println!("Installation directory: {}", install_path.display());
    
    // Create installation directory
    if !install_path.exists() {
        fs::create_dir_all(&install_path)?;
        println!("Created installation directory");
    }
    
    // Copy current executable to installation directory
    let current_exe = env::current_exe()?;
    let target_exe = install_path.join("lemo.exe");
    
    println!("Copying executable...");
    fs::copy(&current_exe, &target_exe)?;
    println!("Executable copied successfully");
    
    // Add to system PATH using PowerShell
    println!("Adding to system PATH...");
    
    let install_path_str = install_path.to_string_lossy().to_string();
    let ps_script = format!(
        r#"
        $installPath = '{}'
        $currentPath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
        if ($currentPath -notlike "*$installPath*") {{
            $newPath = "$currentPath;$installPath"
            [Environment]::SetEnvironmentVariable('Path', $newPath, 'Machine')
            Write-Host 'Successfully added to system PATH'
        }} else {{
            Write-Host 'Already in system PATH'
        }}
        "#,
        install_path_str
    );
    
    // CREATE_NO_WINDOW flag to hide the PowerShell window
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &ps_script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;
    
    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!();
        println!("====================================");
        println!("Installation completed successfully!");
        println!("====================================");
        println!();
        println!("Installation location: {}", install_path.display());
        println!();
        println!("Usage:");
        println!("  lemo fix-icon-cache    # Fix icon cache");
        println!("  lemo clean-temp        # Clean temporary files");
        println!("  lemo install           # Install to system");
        println!("  lemo uninstall         # Uninstall from system");
        println!();
        println!("NOTE: You may need to restart your terminal for PATH changes to take effect");
        println!();
    } else {
        eprintln!("Failed to add to PATH: {}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("Failed to add to system PATH"));
    }
    
    Ok(())
}

/// Uninstall the application from the system
pub fn uninstall_from_system() -> Result<()> {
    use std::os::windows::process::CommandExt;
    
    println!("Starting uninstallation...");
    
    // Get installation path
    let local_appdata = env::var("LOCALAPPDATA")?;
    let install_path = PathBuf::from(local_appdata).join("lemo");
    let install_path_str = install_path.to_string_lossy().to_string();
    
    // Remove from system PATH using PowerShell
    println!("Removing from system PATH...");
    
    let ps_script = format!(
        r#"
        $installPath = '{}'
        $currentPath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
        if ($currentPath -like "*$installPath*") {{
            $newPath = ($currentPath -split ';' | Where-Object {{ $_ -ne $installPath }}) -join ';'
            [Environment]::SetEnvironmentVariable('Path', $newPath, 'Machine')
            Write-Host 'Successfully removed from system PATH'
        }} else {{
            Write-Host 'Not in system PATH'
        }}
        "#,
        install_path_str
    );
    
    // CREATE_NO_WINDOW flag to hide the PowerShell window
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &ps_script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;
    
    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Warning: Failed to remove from PATH: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Delete installation directory
    if install_path.exists() {
        println!("Deleting installation directory: {}", install_path.display());
        fs::remove_dir_all(&install_path)?;
        println!("Installation directory deleted");
    } else {
        println!("Installation directory does not exist");
    }
    
    println!();
    println!("====================================");
    println!("Uninstallation completed!");
    println!("====================================");
    println!();
    
    Ok(())
}

