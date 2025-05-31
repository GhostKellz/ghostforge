use std::fs;
use std::path::Path;

pub fn run() {
    let repo_dir = "repo";
    if !Path::new(repo_dir).exists() {
        if let Err(e) = fs::create_dir(repo_dir) {
            println!("Failed to create repo directory: {}", e);
            return;
        }
    }
    let entries = match fs::read_dir(".") {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut found = false;
    for entry in entries.flatten() {
        let fname = entry.file_name();
        if fname.to_string_lossy().ends_with(".pkg.tar.zst") {
            let dest = Path::new(repo_dir).join(&fname);
            match fs::copy(entry.path(), &dest) {
                Ok(_) => println!("Published {} to repo/", fname.to_string_lossy()),
                Err(e) => println!("Failed to publish {}: {}", fname.to_string_lossy(), e),
            }
            found = true;
        }
    }
    if !found {
        println!("No .pkg.tar.zst files found to publish.");
    }
}
