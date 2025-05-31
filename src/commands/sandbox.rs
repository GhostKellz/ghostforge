use std::env;
use std::process::Command;

pub fn run() {
    // Run build in a temporary directory using bubblewrap if available
    let build_dir = env::current_dir().unwrap();
    let bwrap = Command::new("which")
        .arg("bwrap")
        .output()
        .ok()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if bwrap {
        println!("Running build in sandbox using bubblewrap...");
        let status = Command::new("bwrap")
            .arg("--ro-bind")
            .arg(&build_dir)
            .arg("/src")
            .arg("--dev")
            .arg("/dev")
            .arg("--proc")
            .arg("/proc")
            .arg("--unshare-all")
            .arg("--die-with-parent")
            .arg("--chdir")
            .arg("/src")
            .arg("forge")
            .arg("build")
            .status();
        match status {
            Ok(s) if s.success() => println!("Sandboxed build succeeded."),
            Ok(s) => println!("Sandboxed build failed with status: {}", s),
            Err(e) => println!("Failed to run bubblewrap: {}", e),
        }
    } else {
        println!("bubblewrap not found. Running build without sandbox.");
        let status = Command::new("forge").arg("build").status();
        match status {
            Ok(s) if s.success() => println!("Build succeeded."),
            Ok(s) => println!("Build failed with status: {}", s),
            Err(e) => println!("Failed to run build: {}", e),
        }
    }
}
