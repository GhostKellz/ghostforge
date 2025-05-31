use crate::commands::ui;
use crate::manifest::Manifest;

pub fn run() {
    match Manifest::detect() {
        Some(manifest) => {
            ui::print_info("Manifest info:");
            if let Some(data) = &manifest.data {
                ui::print_info(&format!("  Name: {}", data.name));
                ui::print_info(&format!("  Version: {}", data.version));
                if let Some(author) = &data.author {
                    ui::print_info(&format!("  Author: {}", author));
                }
                if let Some(license) = &data.license {
                    ui::print_info(&format!("  License: {}", license));
                }
                if let Some(source) = &data.source {
                    ui::print_info(&format!("  Source: {}", source));
                }
                if let Some(checksum) = &data.checksum {
                    ui::print_info(&format!("  Checksum: {}", checksum));
                }
                if let Some(build) = &data.build {
                    ui::print_info(&format!("  Build: {}", build));
                }
                if let Some(install) = &data.install {
                    ui::print_info(&format!("  Install: {}", install));
                }
            } else {
                ui::print_info("  (No manifest data parsed)");
            }
        }
        None => {
            ui::print_error("No PKGBUILD or ghostpkg.toml found in the current directory.");
        }
    }
}
