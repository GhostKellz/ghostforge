use std::fs;
use std::path::Path;
use crate::commands::ui;

pub fn run() {
    // Remove common build artifacts and temp directories
    let paths = ["target", "build", "pkg", "*.tar.zst", "*.tar", "*.pkg.tar.zst"];
    for p in &paths {
        if p.contains("*") {
            // Glob pattern
            match glob::glob(p) {
                Ok(paths) => {
                    for entry in paths.flatten() {
                        let _ = fs::remove_file(&entry);
                    }
                }
                Err(_) => {}
            }
        } else if Path::new(p).exists() {
            let _ = fs::remove_dir_all(p);
            let _ = fs::remove_file(p);
        }
    }
    ui::print_success("Cleaned build artifacts and temp files.");
}