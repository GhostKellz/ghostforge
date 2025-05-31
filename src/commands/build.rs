use std::process::Command;
use std::fs;
use rayon::prelude::*;
use crate::commands::ui;

pub fn run() {
    // Example: parallel build for multiple PKGBUILDs/ghostpkg.toml in subdirs
    let entries = match fs::read_dir(".") {
        Ok(e) => e,
        Err(_) => return,
    };
    let entries: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .collect();
    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        if path.join("PKGBUILD").exists() || path.join("ghostpkg.toml").exists() {
            ui::print_info(&format!("Parallel building in {:?}", path));
            let status = Command::new("forge")
                .current_dir(&path)
                .arg("build")
                .status();
            match status {
                Ok(s) if s.success() => ui::print_success(&format!("Build succeeded in {:?}", path)),
                Ok(s) => ui::print_error(&format!("Build failed in {:?} with status: {}", path, s)),
                Err(e) => ui::print_error(&format!("Failed to run build in {:?}: {}", path, e)),
            }
        }
    });
    ui::print_info("Parallel build complete.");

    match crate::manifest::Manifest::detect() {
        Some(manifest) => {
            ui::print_info(&format!("Detected manifest: {} (at {})", manifest.describe(), manifest.path));
            match manifest.manifest_type {
                crate::manifest::ManifestType::GhostpkgToml => {
                    if let Some(data) = &manifest.data {
                        if let Some(build_cmd) = &data.build {
                            ui::print_info(&format!("Running build command: {}", build_cmd));
                            let status = if cfg!(target_os = "windows") {
                                Command::new("cmd").arg("/C").arg(build_cmd).status()
                            } else {
                                Command::new("sh").arg("-c").arg(build_cmd).status()
                            };
                            match status {
                                Ok(s) if s.success() => ui::print_success("Build succeeded."),
                                Ok(s) => ui::print_error(&format!("Build failed with status: {}", s)),
                                Err(e) => ui::print_error(&format!("Failed to run build command: {}", e)),
                            }
                        } else {
                            ui::print_error("No build command found in manifest.");
                        }
                    }
                }
                crate::manifest::ManifestType::PKGBUILD => {
                    ui::print_info("Running PKGBUILD build() function...");
                    let status = Command::new("bash")
                        .arg("-c")
                        .arg("source PKGBUILD; build")
                        .status();
                    match status {
                        Ok(s) if s.success() => ui::print_success("PKGBUILD build() succeeded."),
                        Ok(s) => ui::print_error(&format!("PKGBUILD build() failed with status: {}", s)),
                        Err(e) => ui::print_error(&format!("Failed to run PKGBUILD build(): {}", e)),
                    }
                }
            }
        }
        None => {
            ui::print_error("No PKGBUILD or ghostpkg.toml found in the current directory.");
        }
    }
}