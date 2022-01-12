use crate::common::*;
use serde::Deserialize;
use slug::slugify;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct ForeignDependency {
    git: String,
    commit: String,
    copy: GlobCopySet,
}

impl ForeignDependency {
    pub fn update<P: AsRef<Path>>(&self, out_dir: P) -> bool {
        let updated = git::update_repository(&self.git, &self.commit, self.clone_path(&out_dir));

        if updated {
            copy_globs(&self.copy, &self.clone_path(&out_dir), &out_dir);
        }

        updated
    }

    pub fn clone_path<P: AsRef<Path>>(&self, out_dir: P) -> PathBuf {
        out_dir.as_ref().join(slugify(&self.git))
    }
}

pub mod manifest {
    use crate::fdependencies::ForeignDependency;
    use serde::Deserialize;
    use std::{collections::BTreeMap, path::Path};

    #[derive(Debug, Deserialize)]
    pub struct ForeignDependenciesDetail(BTreeMap<String, ForeignDependency>);

    impl ForeignDependenciesDetail {
        pub fn update_all<P: AsRef<Path>>(&self, out_dir: P) {
            for (_, detail) in self.0.iter() {
                let updated = detail.update(&out_dir);

                if !updated {
                    continue;
                }
            }
        }
    }
}
