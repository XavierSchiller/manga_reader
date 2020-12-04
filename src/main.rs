use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_not_directory(entry: &DirEntry, path: &Path) -> bool {
    if entry.path() == path {
        return true;
    } else {
        return !entry.file_type().is_dir();
    }
}

fn main() {
    let imp_path = Path::new("imported");
    WalkDir::new("imported")
        .into_iter()
        .filter_entry(|e| is_not_directory(e, imp_path))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()))
}
