use crate::manifest::Manifest;
use std::fs;

pub fn run() {
    match Manifest::detect() {
        Some(manifest) => {
            if let Some(data) = &manifest.data {
                // Try to extract dependencies from ghostpkg.toml
                if let Some(source) = &data.source {
                    println!("Source: {}", source);
                }
                // For PKGBUILD, try to extract depends array
                if manifest.path == "PKGBUILD" {
                    let content = fs::read_to_string("PKGBUILD").unwrap_or_default();
                    for line in content.lines() {
                        if line.trim_start().starts_with("depends=") {
                            println!("{}", line.trim());
                        }
                    }
                }
            } else if manifest.path == "PKGBUILD" {
                let content = fs::read_to_string("PKGBUILD").unwrap_or_default();
                for line in content.lines() {
                    if line.trim_start().starts_with("depends=") {
                        println!("{}", line.trim());
                    }
                }
            } else {
                println!("No dependency information found.");
            }
        }
        None => {
            println!("No PKGBUILD or ghostpkg.toml found in the current directory.");
        }
    }
}
