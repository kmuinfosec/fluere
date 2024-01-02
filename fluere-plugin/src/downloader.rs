use crate::util::home_cache_path;
use git2;
use std::io;
use std::path::Path;

pub fn download_plugin_from_github(repo_name: &str) -> Result<(), std::io::Error> {
    let url = format!("https://github.com/{}.git", repo_name);
    let path = home_cache_path()?;
    let cd_cmd = format!("cd {}", path.display());
    if !path?.exists() {
        std::fs::create_dir_all(path.clone())?;
    }
    let repository_path = Path::new(&path);

    match git2::Repository::open(repository_path) {
        Ok(repo) => {
            let mut origin = repo.find_remote("origin").or_else(|_| {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Remote 'origin' does not exist",
                ))
            })?;
            origin
                .fetch(&["refs/heads/*:refs/heads/*"], None, None)
                .map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, format!("fetch failed: {}", e))
                })?;
        }
        Err(_) => {
            git2::Repository::clone(&url, repository_path).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("clone failed: {}", e))
            })?;
        }
    };

    Ok(())
}
