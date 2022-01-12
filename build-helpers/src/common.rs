use glob::glob;
use std::{fs, path::Path};

pub type GlobCopySet = Vec<(String, String)>;

pub fn copy_globs<P: AsRef<Path>, Q: AsRef<Path>>(detail: &GlobCopySet, src_dir: P, dest_dir: Q) {
    for (src_glob, dest_part) in detail {
        let dest_dir = dest_dir.as_ref().join(dest_part);

        if dest_dir.exists() {
            fs::remove_dir_all(&dest_dir).unwrap();
        }

        for file_path in glob(src_dir.as_ref().join(src_glob).to_str().unwrap()).unwrap() {
            let file_path = file_path.unwrap();
            let dest_path = dest_dir.join(file_path.file_name().unwrap());

            fs::create_dir_all(&dest_dir).unwrap();
            fs::copy(&file_path, &dest_path).unwrap();
        }
    }
}
