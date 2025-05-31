use std::process::Command;
use std::fs;

pub fn create_package(build_dir: &str, output_path: &str) -> std::io::Result<()> {
    // Use tar and zstd to create a .pkg.tar.zst
    let tar_path = format!("{}.tar", output_path);
    let status = Command::new("tar")
        .arg("-cf")
        .arg(&tar_path)
        .arg("-C")
        .arg(build_dir)
        .arg(".")
        .status()?;
    if !status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "tar failed"));
    }
    let status = Command::new("zstd")
        .arg("-f")
        .arg(&tar_path)
        .status()?;
    if !status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "zstd failed"));
    }
    fs::remove_file(&tar_path)?;
    Ok(())
}