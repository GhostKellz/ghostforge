use crate::manifest::Manifest;
use std::process::Command;

pub fn run() {
    match Manifest::detect() {
        Some(manifest) => {
            if let Some(data) = &manifest.data {
                if let Some(test_cmd) = data.build.as_ref().filter(|s| s.contains("test")) {
                    println!("Running test command: {}", test_cmd);
                    let status = if cfg!(target_os = "windows") {
                        Command::new("cmd").arg("/C").arg(test_cmd).status()
                    } else {
                        Command::new("sh").arg("-c").arg(test_cmd).status()
                    };
                    match status {
                        Ok(s) if s.success() => println!("Test succeeded."),
                        Ok(s) => println!("Test failed with status: {}", s),
                        Err(e) => println!("Failed to run test command: {}", e),
                    }
                } else {
                    println!(
                        "No test command found in manifest. (Convention: build = 'cargo test', etc.)"
                    );
                }
            } else {
                println!("No manifest data available (PKGBUILD test not implemented).");
            }
        }
        None => {
            println!("No PKGBUILD or ghostpkg.toml found in the current directory.");
        }
    }
}
