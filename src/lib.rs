use std::path::Path;
use gix::features::progress;

#[cfg(windows)]
pub fn bytes2path(b: &[u8]) -> &std::path::Path {
    use std::str;
    std::path::Path::new(str::from_utf8(b).unwrap())
}

pub fn gitoxide(dir: &Path) -> bool {
    let repo = gix::discover(dir).unwrap();
    let status = repo.status(progress::Discard).unwrap();
    status.into_index_worktree_iter(None).unwrap()
        .for_each(|a| {
            let item = a.unwrap();
            println!("{:?}", item.rela_path());
        });

    return false;
}

pub fn is_dirty(dir: &Path) -> Result<Option<Vec<String>>, git2::Error> {
    let repo = git2::Repository::discover(dir)?;

    let mut entries = Vec::new();
    let mut options = git2::StatusOptions::new();
    options
        .show(git2::StatusShow::IndexAndWorkdir)
        .include_untracked(true);
    let statuses = repo.statuses(Some(&mut options))?;
    let dirty_tree = !statuses.is_empty();
    if dirty_tree {
        for status in statuses.iter() {
            let path = bytes2path(status.path_bytes());
            entries.push(format!("{} ({:?})", path.display(), status.status()));
        }
    }

    if entries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(entries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_gix() {
        gitoxide(Path::new("."));
    }

    #[test]
    fn check_upstream() {
        match is_dirty(Path::new(".")) {
            Ok(None) => {}
            Ok(Some(entries)) => {
                println!("{}", entries.join("\n"));
            }
            Err(e) => panic!("Error checking dirty state: {}", e),
        }
    }
}
