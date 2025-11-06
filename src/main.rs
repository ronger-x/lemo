use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

mod utils;
use utils::*;

#[derive(Parser)]
#[command(name = "lemo")]
#[command(author = "ronger")]
#[command(version = "0.1.0")]
#[command(about = "Windows System Toolkit with TUI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    FixIconCache { #[arg(short, long, default_value_t = true)] restart_explorer: bool },
    CleanTemp { #[arg(short, long)] include_user: bool },
    SysInfo,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        return run_cli_mode(command);
    }

    if !is_admin() {
        println!("Administrator privileges required. Requesting elevation...");
        run_as_admin()?;
        return Ok(());
    }

    run_tui()
}

fn run_cli_mode(command: Commands) -> Result<()> {
    match command {
        Commands::FixIconCache { restart_explorer } => fix_icon_cache(restart_explorer)?,
        Commands::CleanTemp { include_user } => clean_temp(include_user)?,
        Commands::SysInfo => show_sys_info()?,
    }
    Ok(())
}

fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut selected = 0;
    let items = vec!["🔧 Fix Icon Cache", "🧹 Clean Temp Files", "💻 System Info", "📊 Real-time Monitor", "➡️ Exit"];

    loop {
        terminal.draw(|f| ui(f, selected, &items))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => selected = (selected + 1) % items.len(),
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = if selected > 0 { selected - 1 } else { items.len() - 1 }
                    }
                    KeyCode::Enter => {
                        match selected {
                            0 => {
                                execute_with_pager(terminal, || fix_icon_cache_with_output(true))?;
                            }
                            1 => {
                                execute_with_pager(terminal, || clean_temp_with_output(false))?;
                            }
                            2 => {
                                execute_with_pager(terminal, || show_sys_info_with_output())?;
                            }
                            3 => {
                                show_realtime_monitor(terminal)?;
                            }
                            4 => break,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

// 执行命令并使用可滚动查看器
fn execute_with_pager<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    func: impl FnOnce() -> Result<String>,
) -> Result<()> {
    // 捕获输出
    let output = func()?;
    let lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();
    
    // 显示可滚动查看器
    show_scrollable_viewer(terminal, &lines)?;
    
    Ok(())
}

// 可滚动查看器
fn show_scrollable_viewer<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    lines: &[String],
) -> Result<()> {
    let mut scroll: usize = 0;
    
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());
            
            // 计算可见行数
            let visible_height = chunks[0].height.saturating_sub(2) as usize; // 减去边框
            
            // 创建可见内容
            let visible_lines: Vec<Line> = lines
                .iter()
                .skip(scroll)
                .take(visible_height)
                .map(|s| Line::from(s.clone()))
                .collect();
            
            let paragraph = Paragraph::new(visible_lines)
                .block(
                    Block::default()
                        .title(format!(" Output (Line {}/{}) ", scroll + 1, lines.len()))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                )
                .style(Style::default().fg(Color::White))
                .wrap(Wrap { trim: false });
            
            f.render_widget(paragraph, chunks[0]);
            
            // 底部提示
            let footer = Paragraph::new("↑/↓: Scroll | Home/End: First/Last | Q/Esc/Enter: Return to menu")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[1]);
        })?;
        
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        let visible_height = terminal.size()?.height.saturating_sub(5) as usize;
                        let max_scroll = lines.len().saturating_sub(visible_height);
                        if scroll < max_scroll {
                            scroll += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if scroll > 0 {
                            scroll -= 1;
                        }
                    }
                    KeyCode::PageDown => {
                        let visible_height = terminal.size()?.height.saturating_sub(5) as usize;
                        let max_scroll = lines.len().saturating_sub(visible_height);
                        scroll = (scroll + visible_height).min(max_scroll);
                    }
                    KeyCode::PageUp => {
                        let visible_height = terminal.size()?.height.saturating_sub(5) as usize;
                        scroll = scroll.saturating_sub(visible_height);
                    }
                    KeyCode::Home => {
                        scroll = 0;
                    }
                    KeyCode::End => {
                        let visible_height = terminal.size()?.height.saturating_sub(5) as usize;
                        scroll = lines.len().saturating_sub(visible_height);
                    }
                    _ => {}
                }
            }
        }
    }
    
    terminal.clear()?;
    Ok(())
}

// 实时系统监控仪表盘
fn show_realtime_monitor<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    use sysinfo::{System, Networks};
    
    let mut sys = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(1000); // 1秒刷新一次
    
    loop {
        // 定期刷新系统信息
        if last_update.elapsed() >= update_interval {
            sys.refresh_cpu_usage();
            sys.refresh_memory();
            networks.refresh(); // 刷新网络数据以计算速度
            last_update = Instant::now();
        }
        
        terminal.draw(|f| {
            render_monitor_ui(f, &sys, &networks);
        })?;
        
        // 非阻塞事件检测
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    terminal.clear()?;
    Ok(())
}

// 渲染监控 UI（Grid 布局）
fn render_monitor_ui(f: &mut Frame, sys: &sysinfo::System, networks: &sysinfo::Networks) {
    // 主布局：顶部标题 + 中间内容 + 底部提示
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 标题
            Constraint::Min(0),     // 内容
            Constraint::Length(3),  // 底部提示
        ])
        .split(f.area());
    
    // 标题
    let header = Paragraph::new("📊 Real-time System Monitor")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, main_chunks[0]);
    
    // 内容区域：左右分栏
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // 左侧：CPU + 内存 + 磁盘
            Constraint::Percentage(50),  // 右侧：系统信息 + GPU + 网络
        ])
        .split(main_chunks[1]);
    
    // 左侧：上中下分割（CPU + 内存 + 磁盘）
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),  // CPU
            Constraint::Min(3),  // 内存
            Constraint::Min(0),  // 磁盘
        ])
        .split(content_chunks[0]);
    
    // 右侧：上中下分割（系统信息 + GPU + 网络）
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(6),  // 系统信息
            Constraint::Min(6),  // GPU
            Constraint::Min(0),  // 网络
        ])
        .split(content_chunks[1]);
    
    // 渲染 CPU 信息
    render_cpu_info(f, sys, left_chunks[0]);
    
    // 渲染内存信息
    render_memory_info(f, sys, left_chunks[1]);
    
    // 渲染磁盘信息
    render_disk_info(f, left_chunks[2]);
    
    // 渲染系统基本信息
    render_system_info(f, sys, right_chunks[0]);
    
    // 渲染 GPU 和温度信息
    render_gpu_temperature_info(f, right_chunks[1]);
    
    // 渲染网络信息
    render_network_info(f, networks, right_chunks[2]);
    
    // 底部提示
    let footer = Paragraph::new("Press Q/Esc/Enter to return to menu | Updates every 1 second")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, main_chunks[2]);
}

// 渲染 CPU 信息
fn render_cpu_info(f: &mut Frame, sys: &sysinfo::System, area: Rect) {
    let total_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
        / sys.cpus().len() as f32;
    
    let cpu_brand = sys.cpus().first()
        .map(|cpu| {
            let brand = cpu.brand();
            // 简化CPU名称显示
            if brand.len() > 35 {
                format!("{}...", &brand[..32])
            } else {
                brand.to_string()
            }
        })
        .unwrap_or_else(|| "Unknown".to_string());
    
    let gauge_color = if total_usage > 80.0 {
        Color::Red
    } else if total_usage > 50.0 {
        Color::Yellow
    } else {
        Color::Green
    };
    
    // 使用紧凑的 Block 样式
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!(" 🔧 CPU: {} ", cpu_brand))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .gauge_style(Style::default().fg(gauge_color).add_modifier(Modifier::BOLD))
        .percent(total_usage as u16)
        .label(format!("{:.1}%", total_usage));
    
    f.render_widget(gauge, area);
}

// 渲染内存信息
fn render_memory_info(f: &mut Frame, sys: &sysinfo::System, area: Rect) {
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let usage_percent = (used_mem / total_mem) * 100.0;
    
    let gauge_color = if usage_percent > 80.0 {
        Color::Red
    } else if usage_percent > 60.0 {
        Color::Yellow
    } else {
        Color::Green
    };
    
    // 使用更简洁的标题
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!(" 💾 Memory: {:.1}/{:.1} GB ", used_mem, total_mem))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .gauge_style(Style::default().fg(gauge_color).add_modifier(Modifier::BOLD))
        .percent(usage_percent as u16)
        .label(format!("{:.1}%", usage_percent));
    
    f.render_widget(gauge, area);
}

// 渲染网络信息
fn render_network_info(f: &mut Frame, networks: &sysinfo::Networks, area: Rect) {
    let mut network_lines = Vec::new();
    
    if networks.is_empty() {
        network_lines.push(Line::from(Span::styled(
            "No network interfaces detected",
            Style::default().fg(Color::Yellow)
        )));
    } else {
        // 收集有实际网络活动的接口（排除虚拟网卡和无活动接口）
        let mut active_networks: Vec<_> = networks.iter()
            .filter_map(|(interface_name, data)| {
                let name_lower = interface_name.to_lowercase();
                
                // 过滤虚拟网卡和回环接口
                if name_lower.contains("loopback") 
                    || name_lower.contains("vmware") 
                    || name_lower.contains("virtualbox")
                    || name_lower.contains("vboxnet")
                    || name_lower.starts_with("lo")
                {
                    return None;
                }
                
                // 获取实时网速（字节/秒）
                let received_speed = data.received(); // 字节/秒
                let transmitted_speed = data.transmitted(); // 字节/秒
                
                // 只显示有实际流量的接口（下载或上传速度 > 1 KB/s）
                if received_speed < 1024 && transmitted_speed < 1024 {
                    return None;
                }
                
                Some((interface_name.clone(), received_speed, transmitted_speed))
            })
            .collect();
        
        // 按接口名称排序
        active_networks.sort_by(|a, b| a.0.cmp(&b.0));
        
        // 只显示前2个活跃的网络接口（与任务管理器一致）
        for (interface_name, received_speed, transmitted_speed) in active_networks.iter().take(2) {
            // 截断接口名称以适应显示
            let display_name = if interface_name.len() > 25 {
                format!("{}...", &interface_name[..22])
            } else {
                interface_name.to_string()
            };
            
            // 格式化速度显示
            let download_str = format_speed(*received_speed);
            let upload_str = format_speed(*transmitted_speed);
            
            network_lines.push(Line::from(vec![
                Span::styled(
                    format!("📡 {}", display_name),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]));
            
            network_lines.push(Line::from(vec![
                Span::raw("   ↓ "),
                Span::styled(
                    download_str,
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
                Span::raw("  ↑ "),
                Span::styled(
                    upload_str,
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
                ),
            ]));
        }
        
        if network_lines.is_empty() {
            network_lines.push(Line::from(Span::styled(
                "No active network traffic",
                Style::default().fg(Color::Gray)
            )));
        }
    }
    
    let paragraph = Paragraph::new(network_lines)
        .block(
            Block::default()
                .title(" 🌐 Network (Real-time Speed) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, area);
}

// 格式化网络速度显示
fn format_speed(bytes_per_sec: u64) -> String {
    let speed = bytes_per_sec as f64;
    
    if speed >= 1024.0 * 1024.0 * 1024.0 {
        // GB/s
        format!("{:.2} GB/s", speed / 1024.0 / 1024.0 / 1024.0)
    } else if speed >= 1024.0 * 1024.0 {
        // MB/s
        format!("{:.2} MB/s", speed / 1024.0 / 1024.0)
    } else if speed >= 1024.0 {
        // KB/s
        format!("{:.1} KB/s", speed / 1024.0)
    } else {
        // B/s
        format!("{} B/s", speed as u64)
    }
}

// 渲染磁盘信息
fn render_disk_info(f: &mut Frame, area: Rect) {
    use sysinfo::Disks;
    
    let disks = Disks::new_with_refreshed_list();
    
    // 收集磁盘信息并按盘符排序
    let mut disk_info: Vec<_> = disks.iter().collect();
    disk_info.sort_by(|a, b| {
        let mount_a = a.mount_point().display().to_string();
        let mount_b = b.mount_point().display().to_string();
        mount_a.cmp(&mount_b)
    });
    
    let mut disk_lines = Vec::new();
    
    for disk in disk_info {
        let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_space = total_space - available_space;
        let usage_percent = (used_space / total_space) * 100.0;
        
        let mount_point = disk.mount_point().display().to_string();
        let bar_width = 20;
        let filled = ((usage_percent / 100.0) * bar_width as f64) as usize;
        let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);
        
        let color = if usage_percent > 90.0 {
            Color::Red
        } else if usage_percent > 70.0 {
            Color::Yellow
        } else {
            Color::Green
        };
        
        disk_lines.push(Line::from(vec![
            Span::raw(format!("{:<8}", mount_point)),
            Span::styled(bar, Style::default().fg(color)),
            Span::raw(format!(" {:.0}%", usage_percent)),
        ]));
        
        disk_lines.push(Line::from(format!(
            "        {:.1}/{:.1} GB",
            used_space, total_space
        )));
    }
    
    let paragraph = Paragraph::new(disk_lines)
        .block(
            Block::default()
                .title(" 💿 Disks ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, area);
}

// 渲染 GPU 和温度信息
fn render_gpu_temperature_info(f: &mut Frame, area: Rect) {
    use sysinfo::Components;
    
    let components = Components::new_with_refreshed_list();
    let mut info_lines = Vec::new();
    
    // GPU 信息部分 - 先尝试从温度传感器获取
    let mut has_gpu_temp = false;
    for component in &components {
        let label = component.label().to_lowercase();
        // 扩展GPU检测关键词，包含更多可能的名称
        if label.contains("gpu") 
            || label.contains("video") 
            || label.contains("graphics") 
            || label.contains("vga")
            || label.contains("display")
            || (label.contains("intel") && (label.contains("hd") || label.contains("uhd") || label.contains("iris")))
            || (label.contains("nvidia") || label.contains("geforce") || label.contains("gtx") || label.contains("rtx"))
            || (label.contains("amd") || label.contains("radeon") || label.contains("rx"))
        {
            let temp = component.temperature();
            let color = if temp > 80.0 {
                Color::Red
            } else if temp > 60.0 {
                Color::Yellow
            } else {
                Color::Green
            };
            
            // 提取GPU名称（去掉温度相关的后缀）
            let gpu_name = component.label()
                .replace("temp", "")
                .replace("temperature", "")
                .replace("Temp", "")
                .replace("Temperature", "")
                .trim()
                .to_string();
            
            let display_name = if gpu_name.len() > 22 {
                format!("{}...", &gpu_name[..19])
            } else {
                gpu_name
            };
            
            info_lines.push(Line::from(vec![
                Span::styled("🎮 ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{}: ", display_name),
                    Style::default().fg(Color::White)
                ),
                Span::styled(
                    format!("{:.1}°C", temp),
                    Style::default().fg(color).add_modifier(Modifier::BOLD)
                ),
            ]));
            has_gpu_temp = true;
            break; // 只显示第一个 GPU
        }
    }
    
    if !has_gpu_temp {
        // 尝试从系统信息获取GPU型号（即使没有温度）
        // 这里显示一个占位符，表示检测到GPU但无温度数据
        if is_admin() {
            info_lines.push(Line::from(Span::styled(
                "🎮 GPU: No temp sensor found",
                Style::default().fg(Color::Gray)
            )));
        } else {
            info_lines.push(Line::from(Span::styled(
                "🎮 GPU: Requires admin rights",
                Style::default().fg(Color::Yellow)
            )));
        }
    }
    
    info_lines.push(Line::from("")); // 空行分隔
    
    // 温度信息部分
    if components.is_empty() {
        if is_admin() {
            info_lines.push(Line::from(Span::styled(
                "🌡️  No sensors detected",
                Style::default().fg(Color::Yellow)
            )));
        } else {
            info_lines.push(Line::from(Span::styled(
                "⚠️  Admin rights required",
                Style::default().fg(Color::Red)
            )));
            info_lines.push(Line::from(Span::styled(
                "   for temperature monitoring",
                Style::default().fg(Color::Gray)
            )));
        }
    } else {
        // 显示主要温度传感器（CPU、主板等）
        let mut sensor_count = 0;
        for component in &components {
            let label = component.label().to_lowercase();
            
            // 跳过 GPU（已在上面显示）
            if label.contains("gpu") || label.contains("video") || label.contains("graphics")
                || label.contains("vga") || label.contains("display")
                || (label.contains("intel") && (label.contains("hd") || label.contains("uhd") || label.contains("iris")))
                || (label.contains("nvidia") || label.contains("geforce"))
                || (label.contains("amd") || label.contains("radeon"))
            {
                continue;
            }
            
            // 优先显示 CPU 和主板温度
            if label.contains("cpu") || label.contains("core") || label.contains("package") 
                || label.contains("motherboard") || label.contains("system") {
                
                if sensor_count >= 4 {
                    break; // 最多显示4个传感器
                }
                
                let temp = component.temperature();
                let color = if temp > 80.0 {
                    Color::Red
                } else if temp > 60.0 {
                    Color::Yellow
                } else {
                    Color::Green
                };
                
                // 截断传感器名称
                let display_label = if component.label().len() > 18 {
                    format!("{}...", &component.label()[..15])
                } else {
                    component.label().to_string()
                };
                
                info_lines.push(Line::from(vec![
                    Span::raw(format!("{:<20}", display_label)),
                    Span::styled(
                        format!("{:.1}°C", temp),
                        Style::default().fg(color).add_modifier(Modifier::BOLD)
                    ),
                ]));
                
                sensor_count += 1;
            }
        }
        
        if sensor_count == 0 {
            info_lines.push(Line::from(Span::styled(
                "No CPU/System sensors found",
                Style::default().fg(Color::Gray)
            )));
        }
    }
    
    let paragraph = Paragraph::new(info_lines)
        .block(
            Block::default()
                .title(" � GPU & Temperature ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, area);
}

// 渲染系统基本信息
fn render_system_info(f: &mut Frame, sys: &sysinfo::System, area: Rect) {
    use std::env;
    
    let uptime = sysinfo::System::uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    
    let info_lines = vec![
        Line::from(format!("OS: {}", env::consts::OS)),
        Line::from(format!("Arch: {}", env::consts::ARCH)),
        Line::from(format!(
            "Cores: {} physical, {} logical",
            sys.physical_core_count().unwrap_or(0),
            sys.cpus().len()
        )),
        Line::from(format!("Uptime: {}d {}h {}m", days, hours, minutes)),
    ];
    
    let paragraph = Paragraph::new(info_lines)
        .block(
            Block::default()
                .title(" ℹ️  System Info ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, area);
}

fn ui(f: &mut Frame, selected: usize, items: &[&str]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let header = Paragraph::new("🍋 Lemo - Windows System Toolkit")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == selected {
                Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!(" {}", item)).style(style)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().title("Main Menu").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑/↓: Navigate | Enter: Execute | Q/Esc: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
