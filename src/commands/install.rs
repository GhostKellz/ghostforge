use crate::commands::ui;
use crate::manifest::Manifest;
use std::env;
use std::process::Command;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let manifest_path = args.iter().skip(2).find(|a| a.ends_with(".toml") || a.ends_with("PKGBUILD") || a.ends_with("forge.lua"));
    let manifest = if let Some(path) = manifest_path {
        Manifest::detect_with_path(Some(path))
    } else {
        Manifest::detect()
    };
    match manifest {
        Some(manifest) => {
            ui::print_info(&format!("Detected manifest: {} (at {})", manifest.describe(), manifest.path));
            match manifest.manifest_type {
                crate::manifest::ManifestType::GhostforgeToml | crate::manifest::ManifestType::GhostpkgToml => {
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
                                Ok(s) => ui::print_error(&format!("Install failed with status: {}", s)),
                                Err(e) => ui::print_error(&format!("Failed to run install command: {}", e)),
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
                        Ok(s) => ui::print_error(&format!("PKGBUILD package() failed with status: {}", s)),
                        Err(e) => ui::print_error(&format!("Failed to run PKGBUILD package(): {}", e)),
                    }
                }
                crate::manifest::ManifestType::ForgeLua | crate::manifest::ManifestType::AutoRust => {
                    ui::print_info("Install is not supported for this manifest type. Use a TOML or PKGBUILD manifest for install logic.");
                }
            }
        }
        None => {
            ui::print_error("No PKGBUILD, ghostpkg.toml, or ghostforge.toml found in the current directory or specified path. Try 'ghostforge init' to scaffold a manifest.");
        }
    }
}
