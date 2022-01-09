use crate::git;
use glob::glob;
use serde::Deserialize;
use slug::slugify;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

type ForeignDepsSet = BTreeMap<String, ForeignDependency>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub foreign_dependencies: ForeignDepsSet,
}

#[derive(Debug, Deserialize)]
pub struct ForeignDependency {
    git: String,
    commit: String,
    copy: Vec<(String, String)>,
}

impl ForeignDependency {
    pub fn update<P: AsRef<Path>>(&self, out_dir: P) -> bool {
        let updated = git::update_repository(&self.git, &self.commit, self.clone_path(&out_dir));
        if updated {
            self.copy_files(&out_dir);
        }

        updated
    }

    pub fn clone_path<P: AsRef<Path>>(&self, out_dir: P) -> PathBuf {
        out_dir.as_ref().join(slugify(&self.git))
    }

    fn copy_files<P: AsRef<Path>>(&self, out_dir: P) {
        let clone_path = self.clone_path(&out_dir);

        for (src_glob, dest_dir) in &self.copy {
            let dest_dir = out_dir.as_ref().join(dest_dir);
            if dest_dir.exists() {
                fs::remove_dir_all(&dest_dir).unwrap();
            }

            for file_path in glob(clone_path.join(src_glob).to_str().unwrap()).unwrap() {
                let file_path = file_path.unwrap();
                let dest_path = dest_dir.join(file_path.file_name().unwrap());

                fs::create_dir_all(&dest_dir).unwrap();
                fs::copy(&file_path, &dest_path).unwrap();
            }
        }
    }
}
