use crate::commands::package;
use crate::commands::ui;
use std::fs;
use std::process::Command;

pub fn run() {
    let entries = match fs::read_dir(".") {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let fname = entry.file_name();
        let fname_str = fname.to_string_lossy();
        if fname_str == "PKGBUILD" || fname_str == "ghostpkg.toml" {
            ui::print_info(&format!("Rebuilding package: {}", fname_str));
            let status = Command::new("forge").arg("build").status();
            match status {
                Ok(s) if s.success() => {
                    ui::print_success(&format!("Rebuild succeeded for {}", fname_str));
                    // Example: call create_package after successful build
                    let _ = package::create_package(".", &format!("{}.pkg.tar.zst", fname_str));
                }
                Ok(s) => ui::print_error(&format!(
                    "Rebuild failed for {} with status: {}",
                    fname_str, s
                )),
                Err(e) => ui::print_error(&format!(
                    "Failed to run forge build for {}: {}",
                    fname_str, e
                )),
            }
        }
    }
}
