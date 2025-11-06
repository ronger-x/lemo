// Build script for embedding Windows resources
use std::env;

fn main() {
    // 只在 Windows 平台编译资源文件
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // 嵌入 Windows 资源文件（图标和版本信息）
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}
