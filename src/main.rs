use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use zip::ZipArchive;

fn is_zip_file(entry: &DirEntry) -> bool {
    return entry.path().is_file() && entry.path().extension().unwrap() == "zip";
}

fn read_zip(path: &str) -> Vec<String> {
    let zipfile = File::open(path).unwrap();
    let mut res: Vec<String> = Vec::new();

    let mut archive = ZipArchive::new(zipfile).unwrap();

    for i in 0..archive.len() {
        res.push(archive.by_index(i).unwrap().name().to_owned());
    }

    res.sort_unstable();

    return res;
}

fn main() {
    WalkDir::new("imported")
        .min_depth(1) // LOL I had to read the docs for this one.
        .max_depth(1) // So that it doesn't recurse into the directories I don't want it to.
        .into_iter()
        .filter_entry(|e| is_zip_file(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{:?}", read_zip(x.path().to_str().unwrap())));
    std::process::exit(0);
}
