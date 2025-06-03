use crate::commands::ui;
use std::fs;

pub fn run() {
    let template = r#"name = "ghostscan"
version = "0.1.0"
author = "CK Technology LLC"
license = "MIT"
build = "cargo build --release"
install = "install -Dm755 target/release/ghostscan /usr/bin/ghostscan"
"#;
    if fs::metadata("ghostforge.toml").is_ok() {
        ui::print_error("ghostforge.toml already exists in this directory.");
        return;
    }
    match fs::write("ghostforge.toml", template) {
        Ok(_) => ui::print_success("Created starter ghostforge.toml."),
        Err(e) => ui::print_error(&format!("Failed to create ghostforge.toml: {}", e)),
    }
}
