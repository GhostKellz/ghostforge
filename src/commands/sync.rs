use std::process::Command;

pub fn run() {
    let repo_dir = "repo";
    let remote = std::env::var("FORGE_SYNC_REMOTE").ok();
    if let Some(remote) = remote {
        println!("Syncing repo/ to remote: {}", remote);
        let status = Command::new("rsync")
            .arg("-avz")
            .arg(repo_dir)
            .arg(&remote)
            .status();
        match status {
            Ok(s) if s.success() => println!("Sync complete."),
            Ok(s) => println!("Sync failed with status: {}", s),
            Err(e) => println!("Failed to run rsync: {}", e),
        }
    } else {
        println!("No FORGE_SYNC_REMOTE set. To sync, set FORGE_SYNC_REMOTE=dest e.g. user@host:/path");
    }
}