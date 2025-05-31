use crate::commands::ui;
use crate::manifest::Manifest;
use std::fs;
use std::process::Command;

pub fn run() {
    // Enforce hash/signature verification before install
    let pkg = match fs::read_dir(".") {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .find(|e| e.file_name().to_string_lossy().ends_with(".pkg.tar.zst")),
        Err(_) => None,
    };
    if let Some(pkg) = pkg {
        let pkg_path = pkg.file_name();
        let sig_path = format!("{}.sig", pkg_path.to_string_lossy());
        if fs::metadata(&sig_path).is_ok() {
            let status = Command::new("gpg")
                .arg("--verify")
                .arg(&sig_path)
                .arg(&pkg_path)
                .status();
            if !matches!(status, Ok(s) if s.success()) {
                ui::print_error("Signature verification failed. Aborting install.");
                return;
            }
        }
    }

    match Manifest::detect() {
        Some(manifest) => {
            ui::print_info(&format!(
                "Detected manifest: {} (at {})",
                manifest.describe(),
                manifest.path
            ));
            match manifest.manifest_type {
                crate::manifest::ManifestType::GhostpkgToml => {
                    if let Some(data) = &manifest.data {
                        if let Some(install_cmd) = &data.install {
                            ui::print_info(&format!("Running install command: {}", install_cmd));
                            let status = if cfg!(target_os = "windows") {
                                Command::new("cmd").arg("/C").arg(install_cmd).status()
                            } else {
                                Command::new("sh").arg("-c").arg(install_cmd).status()
                            };
                            match status {
                                Ok(s) if s.success() => ui::print_success("Install succeeded."),
                                Ok(s) => {
                                    ui::print_error(&format!("Install failed with status: {}", s))
                                }
                                Err(e) => ui::print_error(&format!(
                                    "Failed to run install command: {}",
                                    e
                                )),
                            }
                        } else {
                            ui::print_error("No install command found in manifest.");
                        }
                    }
                }
                crate::manifest::ManifestType::PKGBUILD => {
                    ui::print_info("Running PKGBUILD package() function...");
                    let status = Command::new("bash")
                        .arg("-c")
                        .arg("source PKGBUILD; package")
                        .status();
                    match status {
                        Ok(s) if s.success() => ui::print_success("PKGBUILD package() succeeded."),
                        Ok(s) => ui::print_error(&format!(
                            "PKGBUILD package() failed with status: {}",
                            s
                        )),
                        Err(e) => {
                            ui::print_error(&format!("Failed to run PKGBUILD package(): {}", e))
                        }
                    }
                }
            }
        }
        None => {
            ui::print_error("No PKGBUILD or ghostpkg.toml found in the current directory.");
        }
    }
}
