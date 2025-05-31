use std::fs;

pub fn run() {
    let template = r#"name = "example"
version = "0.1.0"
author = "Your Name <you@example.com>"
license = "MIT"
build = "make"
install = "make install"
source = "https://example.com/source.tar.gz"
checksum = "sha256:..."
"#;
    if fs::metadata("ghostpkg.toml").is_ok() {
        println!("ghostpkg.toml already exists in this directory.");
        return;
    }
    match fs::write("ghostpkg.toml", template) {
        Ok(_) => println!("Created starter ghostpkg.toml."),
        Err(e) => println!("Failed to create ghostpkg.toml: {}", e),
    }
}