// Utility functions module
use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::System;
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

// Fix icon cache (returns output as String for TUI)
pub fn fix_icon_cache_with_output(restart_explorer: bool) -> Result<String> {
    let mut output = String::new();
    output.push_str("🔧 Fixing icon cache...\n");

    output.push_str("⏳ Closing Windows Explorer...\n");
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
                output.push_str(&format!("✅ Deleted: {:?}\n", file));
                deleted_count += 1;
            }
            Err(e) => {
                output.push_str(&format!("⚠️  Skipped: {:?} ({})\n", file, e));
                skipped_count += 1;
            }
        }
    }

    output.push_str(&format!(
        "\n📊 Summary: Deleted {} files, Skipped {} files\n",
        deleted_count, skipped_count
    ));

    if restart_explorer {
        output.push_str("🔄 Restarting Windows Explorer...\n");
        Command::new("explorer.exe").spawn()?;
        output.push_str("✨ Fix completed! Desktop will restore in a few seconds.\n");
        thread::sleep(Duration::from_secs(3));
    } else {
        output.push_str("✨ Fix completed! Please restart Explorer manually.\n");
    }

    Ok(output)
}

// Clean temporary files
pub fn clean_temp(include_user: bool) -> Result<()> {
    println!("🧹 Cleaning temporary files...");
    println!("═══════════════════════════════════════════════════");

    let mut total_deleted = 0;
    let mut total_failed = 0;
    let mut total_size_freed: u64 = 0;

    // 清理 Windows Temp 目录
    let windows_temp = PathBuf::from(r"C:\Windows\Temp");
    if windows_temp.exists() {
        println!("\n📁 Cleaning Windows temp directory: {}", windows_temp.display());
        let (deleted, failed, size) = clean_directory(&windows_temp)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;
        println!(
            "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
            deleted,
            failed,
            size as f64 / 1024.0 / 1024.0
        );
    }

    // 清理 Windows Prefetch
    let prefetch = PathBuf::from(r"C:\Windows\Prefetch");
    if prefetch.exists() {
        println!("\n📁 Cleaning Windows prefetch: {}", prefetch.display());
        let (deleted, failed, size) = clean_directory(&prefetch)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;
        println!(
            "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
            deleted,
            failed,
            size as f64 / 1024.0 / 1024.0
        );
    }

    // 清理系统驱动器临时文件（限制深度的递归搜索）
    println!("\n📁 Scanning system drive for temp files (this may take a while)...");
    let system_drive = env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
    let extensions = vec!["tmp", "log", "gid", "chk", "old", "bak", "_mp"];
    
    let mut last_print = 0;
    let mut progress_callback = |_path: &str, deleted: usize, _failed: usize, size: u64| {
        // 每 100 个文件输出一次进度
        if deleted > 0 && deleted % 100 == 0 && deleted != last_print {
            println!(
                "   ⏳ Progress: {} deleted, {:.2} MB freed...",
                deleted,
                size as f64 / 1024.0 / 1024.0
            );
            last_print = deleted;
        }
    };
    
    let (deleted, failed, size) = clean_files_by_extension_with_progress(
        &PathBuf::from(&system_drive),
        &extensions,
        &mut progress_callback,
        0,
    )?;
    
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;
    println!(
        "   ✅ Completed: {} items deleted, {} skipped, {:.2} MB freed",
        deleted,
        failed,
        size as f64 / 1024.0 / 1024.0
    );

    if include_user {
        // 清理用户临时目录
        if let Ok(temp) = env::var("TEMP") {
            let user_temp = PathBuf::from(temp);
            if user_temp.exists() {
                println!("\n📁 Cleaning user temp directory: {}", user_temp.display());
                let (deleted, failed, size) = clean_directory(&user_temp)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                println!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                );
            }
        }

        // 清理用户相关目录
        if let Ok(userprofile) = env::var("USERPROFILE") {
            // Cookies
            let cookies = PathBuf::from(&userprofile).join("Cookies");
            if cookies.exists() {
                println!("\n📁 Cleaning user cookies: {}", cookies.display());
                let (deleted, failed, size) = clean_directory(&cookies)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                println!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                );
            }

            // Recent
            let recent = PathBuf::from(&userprofile).join("Recent");
            if recent.exists() {
                println!("\n📁 Cleaning recent files: {}", recent.display());
                let (deleted, failed, size) = clean_directory(&recent)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                println!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                );
            }

            // IE Temporary Internet Files
            let ie_temp = PathBuf::from(&userprofile).join(r"Local Settings\Temporary Internet Files");
            if ie_temp.exists() {
                println!("\n📁 Cleaning IE temporary files: {}", ie_temp.display());
                let (deleted, failed, size) = clean_directory(&ie_temp)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                println!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                );
            }

            // Local Settings\Temp
            let local_temp = PathBuf::from(&userprofile).join(r"Local Settings\Temp");
            if local_temp.exists() {
                println!("\n📁 Cleaning local temp: {}", local_temp.display());
                let (deleted, failed, size) = clean_directory(&local_temp)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                println!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                );
            }
        }
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("📊 Cleaning summary:");
    println!("   Total deleted: {} items", total_deleted);
    println!("   Total skipped: {} items", total_failed);
    println!(
        "   Freed space: {:.2} MB ({:.2} GB)",
        total_size_freed as f64 / 1024.0 / 1024.0,
        total_size_freed as f64 / 1024.0 / 1024.0 / 1024.0
    );
    println!("═══════════════════════════════════════════════════");
    println!("✨ Cleaning completed!");

    Ok(())
}

// Clean temporary files (returns output as String for TUI)
pub fn clean_temp_with_output(include_user: bool) -> Result<String> {
    let mut output = String::new();
    output.push_str("🧹 Cleaning temporary files...\n");
    output.push_str("═══════════════════════════════════════════════════\n");

    let mut total_deleted = 0;
    let mut total_failed = 0;
    let mut total_size_freed: u64 = 0;

    // 清理 Windows Temp 目录
    let windows_temp = PathBuf::from(r"C:\Windows\Temp");
    if windows_temp.exists() {
        output.push_str(&format!("\n📁 Cleaning Windows temp directory: {}\n", windows_temp.display()));
        let (deleted, failed, size) = clean_directory_with_output(&windows_temp, &mut output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;
        output.push_str(&format!(
            "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
            deleted,
            failed,
            size as f64 / 1024.0 / 1024.0
        ));
    }

    // 清理 Windows Prefetch
    let prefetch = PathBuf::from(r"C:\Windows\Prefetch");
    if prefetch.exists() {
        output.push_str(&format!("\n📁 Cleaning Windows prefetch: {}\n", prefetch.display()));
        let (deleted, failed, size) = clean_directory_with_output(&prefetch, &mut output)?;
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;
        output.push_str(&format!(
            "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
            deleted,
            failed,
            size as f64 / 1024.0 / 1024.0
        ));
    }

    // 清理系统驱动器临时文件（限制深度的递归搜索）
    output.push_str("\n📁 Scanning system drive for temp files (limited depth)...\n");
    let system_drive = env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
    let extensions = vec!["tmp", "log", "gid", "chk", "old", "bak", "_mp"];
    
    let mut scan_deleted = 0;
    let mut scan_failed = 0;
    let mut scan_size = 0u64;
    let mut current_file = String::new();
    
    let mut progress_callback = |path: &str, deleted: usize, failed: usize, size: u64| {
        current_file = path.to_string();
        scan_deleted = deleted;
        scan_failed = failed;
        scan_size = size;
        
        // 每 100 个文件输出一次进度
        if deleted % 100 == 0 && deleted > 0 {
            output.push_str(&format!(
                "   ⏳ Scanned: {} deleted, {} skipped, {:.2} MB freed\n",
                deleted,
                failed,
                size as f64 / 1024.0 / 1024.0
            ));
        }
    };
    
    let (deleted, failed, size) = clean_files_by_extension_with_progress(
        &PathBuf::from(&system_drive),
        &extensions,
        &mut progress_callback,
        0,
    )?;
    
    total_deleted += deleted;
    total_failed += failed;
    total_size_freed += size;
    output.push_str(&format!(
        "   ✅ Completed: {} items deleted, {} skipped, {:.2} MB freed\n",
        deleted,
        failed,
        size as f64 / 1024.0 / 1024.0
    ));

    if include_user {
        // 清理用户临时目录
        if let Ok(temp) = env::var("TEMP") {
            let user_temp = PathBuf::from(temp);
            if user_temp.exists() {
                output.push_str(&format!("\n📁 Cleaning user temp directory: {}\n", user_temp.display()));
                let (deleted, failed, size) = clean_directory_with_output(&user_temp, &mut output)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                output.push_str(&format!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                ));
            }
        }

        // 清理用户相关目录
        if let Ok(userprofile) = env::var("USERPROFILE") {
            // Cookies
            let cookies = PathBuf::from(&userprofile).join("Cookies");
            if cookies.exists() {
                output.push_str(&format!("\n📁 Cleaning user cookies: {}\n", cookies.display()));
                let (deleted, failed, size) = clean_directory_with_output(&cookies, &mut output)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                output.push_str(&format!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                ));
            }

            // Recent
            let recent = PathBuf::from(&userprofile).join("Recent");
            if recent.exists() {
                output.push_str(&format!("\n📁 Cleaning recent files: {}\n", recent.display()));
                let (deleted, failed, size) = clean_directory_with_output(&recent, &mut output)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                output.push_str(&format!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                ));
            }

            // IE Temporary Internet Files
            let ie_temp = PathBuf::from(&userprofile).join(r"Local Settings\Temporary Internet Files");
            if ie_temp.exists() {
                output.push_str(&format!("\n📁 Cleaning IE temporary files: {}\n", ie_temp.display()));
                let (deleted, failed, size) = clean_directory_with_output(&ie_temp, &mut output)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                output.push_str(&format!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                ));
            }

            // Local Settings\Temp
            let local_temp = PathBuf::from(&userprofile).join(r"Local Settings\Temp");
            if local_temp.exists() {
                output.push_str(&format!("\n📁 Cleaning local temp: {}\n", local_temp.display()));
                let (deleted, failed, size) = clean_directory_with_output(&local_temp, &mut output)?;
                total_deleted += deleted;
                total_failed += failed;
                total_size_freed += size;
                output.push_str(&format!(
                    "   Deleted: {} items, Skipped: {}, Freed: {:.2} MB\n",
                    deleted,
                    failed,
                    size as f64 / 1024.0 / 1024.0
                ));
            }
        }
    }

    output.push_str("\n═══════════════════════════════════════════════════\n");
    output.push_str("📊 Cleaning summary:\n");
    output.push_str(&format!("   Total deleted: {} items\n", total_deleted));
    output.push_str(&format!("   Total skipped: {} items\n", total_failed));
    output.push_str(&format!(
        "   Freed space: {:.2} MB ({:.2} GB)\n",
        total_size_freed as f64 / 1024.0 / 1024.0,
        total_size_freed as f64 / 1024.0 / 1024.0 / 1024.0
    ));
    output.push_str("═══════════════════════════════════════════════════\n");
    output.push_str("✨ Cleaning completed!\n");

    Ok(output)
}

// Clean a directory
pub fn clean_directory(dir: &PathBuf) -> Result<(usize, usize, u64)> {
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
                        println!(
                            "   ✅ Deleted: {}",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        );
                    }
                }
                Err(_) => {
                    failed_count += 1;
                    if failed_count <= 3 {
                        println!(
                            "   ⚠️  Skipped: {} (locked or no permission)",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        );
                    }
                }
            }
        }

        if deleted_count > 5 {
            println!("   ... and {} more items deleted", deleted_count - 5);
        }
        if failed_count > 3 {
            println!("   ... and {} more items skipped", failed_count - 3);
        }
    }

    Ok((deleted_count, failed_count, total_size))
}

// Clean a directory (with output to String)
pub fn clean_directory_with_output(dir: &PathBuf, output: &mut String) -> Result<(usize, usize, u64)> {
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
                        output.push_str(&format!(
                            "   ✅ Deleted: {}\n",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                }
                Err(_) => {
                    failed_count += 1;
                    if failed_count <= 3 {
                        output.push_str(&format!(
                            "   ⚠️  Skipped: {} (locked or no permission)\n",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                }
            }
        }

        if deleted_count > 5 {
            output.push_str(&format!("   ... and {} more items deleted\n", deleted_count - 5));
        }
        if failed_count > 3 {
            output.push_str(&format!("   ... and {} more items skipped\n", failed_count - 3));
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
pub fn show_sys_info() -> Result<()> {
    use sysinfo::Disks;

    println!("💻 System Information:");
    println!("═══════════════════════════════════════════════════");

    println!("\n📌 Basic Information:");
    println!("  OS: {}", env::consts::OS);
    println!("  Architecture: {}", env::consts::ARCH);
    println!(
        "  User: {}",
        env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())
    );
    println!(
        "  Computer: {}",
        env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string())
    );

    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n🔧 CPU Information:");
    println!(
        "  Physical cores: {}",
        sys.physical_core_count().unwrap_or(0)
    );
    println!("  Logical cores: {}", sys.cpus().len());

    if let Some(cpu) = sys.cpus().first() {
        println!("  Model: {}", cpu.brand());
        println!("  Frequency: {} MHz", cpu.frequency());
    }

    thread::sleep(Duration::from_millis(500));
    sys.refresh_cpu_usage();
    let total_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
        / sys.cpus().len() as f32;
    println!("  Total usage: {:.2}%", total_usage);

    println!("\n💾 Memory Information:");
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let available_mem = sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    println!("  Total: {:.2} GB", total_mem);
    println!(
        "  Used: {:.2} GB ({:.1}%)",
        used_mem,
        (used_mem / total_mem) * 100.0
    );
    println!("  Available: {:.2} GB", available_mem);

    println!("\n💿 Disk Information:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_space = total_space - available_space;
        let usage_percent = (used_space / total_space) * 100.0;

        println!(
            "  {} - {}",
            disk.name().to_string_lossy(),
            disk.mount_point().display()
        );
        println!("    Total: {:.2} GB", total_space);
        println!("    Used: {:.2} GB ({:.1}%)", used_space, usage_percent);
        println!("    Available: {:.2} GB", available_space);
    }

    println!("\n⏱️  System Uptime:");
    let uptime = System::uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    println!("  {} days {} hours {} minutes", days, hours, minutes);

    println!("\n═══════════════════════════════════════════════════");

    Ok(())
}

// Show system information (returns output as String for TUI)
pub fn show_sys_info_with_output() -> Result<String> {
    use sysinfo::Disks;

    let mut output = String::new();
    output.push_str("💻 System Information:\n");
    output.push_str("═══════════════════════════════════════════════════\n");

    output.push_str("\n📌 Basic Information:\n");
    output.push_str(&format!("  OS: {}\n", env::consts::OS));
    output.push_str(&format!("  Architecture: {}\n", env::consts::ARCH));
    output.push_str(&format!(
        "  User: {}\n",
        env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())
    ));
    output.push_str(&format!(
        "  Computer: {}\n",
        env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string())
    ));

    let mut sys = System::new_all();
    sys.refresh_all();

    output.push_str("\n🔧 CPU Information:\n");
    output.push_str(&format!(
        "  Physical cores: {}\n",
        sys.physical_core_count().unwrap_or(0)
    ));
    output.push_str(&format!("  Logical cores: {}\n", sys.cpus().len()));

    if let Some(cpu) = sys.cpus().first() {
        output.push_str(&format!("  Model: {}\n", cpu.brand()));
        output.push_str(&format!("  Frequency: {} MHz\n", cpu.frequency()));
    }

    thread::sleep(Duration::from_millis(500));
    sys.refresh_cpu_usage();
    let total_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
        / sys.cpus().len() as f32;
    output.push_str(&format!("  Total usage: {:.2}%\n", total_usage));

    output.push_str("\n💾 Memory Information:\n");
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let available_mem = sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    output.push_str(&format!("  Total: {:.2} GB\n", total_mem));
    output.push_str(&format!(
        "  Used: {:.2} GB ({:.1}%)\n",
        used_mem,
        (used_mem / total_mem) * 100.0
    ));
    output.push_str(&format!("  Available: {:.2} GB\n", available_mem));

    output.push_str("\n💿 Disk Information:\n");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_space = total_space - available_space;
        let usage_percent = (used_space / total_space) * 100.0;

        output.push_str(&format!(
            "  {} - {}\n",
            disk.name().to_string_lossy(),
            disk.mount_point().display()
        ));
        output.push_str(&format!("    Total: {:.2} GB\n", total_space));
        output.push_str(&format!("    Used: {:.2} GB ({:.1}%)\n", used_space, usage_percent));
        output.push_str(&format!("    Available: {:.2} GB\n", available_space));
    }

    output.push_str("\n⏱️  System Uptime:\n");
    let uptime = System::uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    output.push_str(&format!("  {} days {} hours {} minutes\n", days, hours, minutes));

    output.push_str("\n═══════════════════════════════════════════════════\n");

    Ok(output)
}

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
        callback("🧹 Cleaning temporary files...".to_string());
        callback("═══════════════════════════════════════════════════".to_string());

        let mut total_deleted = 0;
        let mut total_failed = 0;
        let mut total_size_freed: u64 = 0;

        // 清理 Windows Temp 目录
        let windows_temp = PathBuf::from(r"C:\Windows\Temp");
        if windows_temp.exists() {
            callback(String::new());
            callback(format!("📁 Cleaning Windows temp directory: {}", windows_temp.display()));
            let (deleted, failed, size) = clean_directory_streaming(&windows_temp, &mut callback)?;
            total_deleted += deleted;
            total_failed += failed;
            total_size_freed += size;
            callback(format!(
                "   ✅ Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                deleted,
                failed,
                size as f64 / 1024.0 / 1024.0
            ));
        }

        // 清理 Windows Prefetch
        let prefetch = PathBuf::from(r"C:\Windows\Prefetch");
        if prefetch.exists() {
            callback(String::new());
            callback(format!("📁 Cleaning Windows prefetch: {}", prefetch.display()));
            let (deleted, failed, size) = clean_directory_streaming(&prefetch, &mut callback)?;
            total_deleted += deleted;
            total_failed += failed;
            total_size_freed += size;
            callback(format!(
                "   ✅ Deleted: {} items, Skipped: {}, Freed: {:.2} MB",
                deleted,
                failed,
                size as f64 / 1024.0 / 1024.0
            ));
        }

        // 清理系统驱动器临时文件
        callback(String::new());
        callback("📁 Scanning system drive for temp files...".to_string());
        let system_drive = env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
        let extensions = vec!["tmp", "log", "gid", "chk", "old", "bak", "_mp"];
        
        let mut last_report = 0;
        let mut progress_callback = |_path: &str, deleted: usize, _failed: usize, size: u64| {
            // 每 100 个文件报告一次进度
            if deleted > 0 && deleted % 100 == 0 && deleted != last_report {
                callback(format!(
                    "   ⏳ Progress: {} deleted, {:.2} MB freed...",
                    deleted,
                    size as f64 / 1024.0 / 1024.0
                ));
                last_report = deleted;
            }
        };
        
        let (deleted, failed, size) = clean_files_by_extension_with_progress(
            &PathBuf::from(&system_drive),
            &extensions,
            &mut progress_callback,
            0,
        )?;
        
        total_deleted += deleted;
        total_failed += failed;
        total_size_freed += size;
        callback(format!(
            "   ✅ Completed: {} items deleted, {} skipped, {:.2} MB freed",
            deleted,
            failed,
            size as f64 / 1024.0 / 1024.0
        ));

        callback(String::new());
        callback("═══════════════════════════════════════════════════".to_string());
        callback("📊 Cleaning summary:".to_string());
        callback(format!("   Total deleted: {} items", total_deleted));
        callback(format!("   Total skipped: {} items", total_failed));
        callback(format!(
            "   Freed space: {:.2} MB ({:.2} GB)",
            total_size_freed as f64 / 1024.0 / 1024.0,
            total_size_freed as f64 / 1024.0 / 1024.0 / 1024.0
        ));
        callback("═══════════════════════════════════════════════════".to_string());
        callback("✨ Cleaning completed!".to_string());

        Ok(())
    }
}

// Clean directory with streaming output
fn clean_directory_streaming(
    dir: &PathBuf,
    callback: &mut Box<dyn FnMut(String) + Send>,
) -> Result<(usize, usize, u64)> {
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
                        callback(format!(
                            "   ✅ Deleted: {}",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                }
                Err(_) => {
                    failed_count += 1;
                    if failed_count <= 3 {
                        callback(format!(
                            "   ⚠️  Skipped: {} (locked or no permission)",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                }
            }
        }

        if deleted_count > 5 {
            callback(format!("   ... and {} more items deleted", deleted_count - 5));
        }
        if failed_count > 3 {
            callback(format!("   ... and {} more items skipped", failed_count - 3));
        }
    }

    Ok((deleted_count, failed_count, total_size))
}

// Show system info with streaming output (callback-based)
pub fn show_sys_info_with_streaming() -> impl FnOnce(Box<dyn FnMut(String) + Send>) -> Result<()> + Send + 'static {
    |mut callback: Box<dyn FnMut(String) + Send>| {
        use sysinfo::Disks;

        callback("💻 System Information:".to_string());
        callback("═══════════════════════════════════════════════════".to_string());

        callback(String::new());
        callback("📌 Basic Information:".to_string());
        callback(format!("  OS: {}", env::consts::OS));
        callback(format!("  Architecture: {}", env::consts::ARCH));
        callback(format!(
            "  User: {}",
            env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())
        ));
        callback(format!(
            "  Computer: {}",
            env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string())
        ));

        let mut sys = System::new_all();
        sys.refresh_all();

        callback(String::new());
        callback("🔧 CPU Information:".to_string());
        callback(format!(
            "  Physical cores: {}",
            sys.physical_core_count().unwrap_or(0)
        ));
        callback(format!("  Logical cores: {}", sys.cpus().len()));

        if let Some(cpu) = sys.cpus().first() {
            callback(format!("  Model: {}", cpu.brand()));
            callback(format!("  Frequency: {} MHz", cpu.frequency()));
        }

        thread::sleep(Duration::from_millis(500));
        sys.refresh_cpu_usage();
        let total_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
            / sys.cpus().len() as f32;
        callback(format!("  Total usage: {:.2}%", total_usage));

        callback(String::new());
        callback("💾 Memory Information:".to_string());
        let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_mem = sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        callback(format!("  Total: {:.2} GB", total_mem));
        callback(format!(
            "  Used: {:.2} GB ({:.1}%)",
            used_mem,
            (used_mem / total_mem) * 100.0
        ));
        callback(format!("  Available: {:.2} GB", available_mem));

        callback(String::new());
        callback("💿 Disk Information:".to_string());
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_space = total_space - available_space;
            let usage_percent = (used_space / total_space) * 100.0;

            callback(format!(
                "  {} - {}",
                disk.name().to_string_lossy(),
                disk.mount_point().display()
            ));
            callback(format!("    Total: {:.2} GB", total_space));
            callback(format!("    Used: {:.2} GB ({:.1}%)", used_space, usage_percent));
            callback(format!("    Available: {:.2} GB", available_space));
        }

        callback(String::new());
        callback("⏱️  System Uptime:".to_string());
        let uptime = System::uptime();
        let days = uptime / 86400;
        let hours = (uptime % 86400) / 3600;
        let minutes = (uptime % 3600) / 60;
        callback(format!("  {} days {} hours {} minutes", days, hours, minutes));

        callback(String::new());
        callback("═══════════════════════════════════════════════════".to_string());

        Ok(())
    }
}

