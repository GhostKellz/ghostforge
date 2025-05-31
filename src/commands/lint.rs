use crate::manifest::Manifest;
use crate::commands::ui;

pub fn run() {
    match Manifest::detect() {
        Some(manifest) => {
            if let Some(data) = &manifest.data {
                let mut errors = Vec::new();
                if data.name.trim().is_empty() {
                    errors.push("Missing 'name' field");
                }
                if data.version.trim().is_empty() {
                    errors.push("Missing 'version' field");
                }
                if data.build.is_none() {
                    errors.push("Missing 'build' command");
                }
                if data.install.is_none() {
                    errors.push("Missing 'install' command");
                }
                if errors.is_empty() {
                    ui::print_success("Manifest is valid.");
                } else {
                    ui::print_error("Manifest has errors:");
                    for e in errors {
                        ui::print_error(&format!("- {}", e));
                    }
                }
            } else {
                ui::print_error("No manifest data available (PKGBUILD linting not implemented).");
            }
        }
        None => {
            ui::print_error("No PKGBUILD or ghostpkg.toml found in the current directory.");
        }
    }
}