use cargo_toml::Manifest;
use git2::{build::CheckoutBuilder, Oid, Repository};
use serde::Deserialize;
use slug::slugify;
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

fn update_repository<P: AsRef<Path>>(url: &str, commit: &str, dest_dir: P) -> bool {
    let mut updated = false;
    let repository = match Repository::open(&dest_dir) {
        Ok(repository) => repository,
        Err(_) => {
            updated = true;
            Repository::clone(url, &dest_dir).unwrap()
        }
    };
    let commit_old = repository.head().unwrap().target().unwrap();
    let commit_new = Oid::from_str(commit).unwrap();

    if commit_old != commit_new {
        updated = true;
        repository.set_head_detached(commit_new).unwrap();
        repository
            .checkout_head(Some(&mut CheckoutBuilder::default().force()))
            .unwrap();
    }

    updated
}

type ForeignDepsSet = BTreeMap<String, ForeignDependency>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
    foreign_dependencies: ForeignDepsSet,
}

#[derive(Debug, Deserialize)]
struct ForeignDependency {
    git: String,
    commit: String,
    copy: Vec<(String, String)>,
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest: Manifest<Metadata> = Manifest::from_path_with_metadata("Cargo.toml").unwrap();

    println!("Output directory: {:?}", out_dir);
    println!("Manifest: {:#?}", manifest);

    // let flag_icons_metadata = metadata["flag-icons"].as_table().unwrap();
    // let flag_icons_git = flag_icons_metadata["git"].as_str().unwrap();
    // let flag_icons_commit = flag_icons_metadata["commit"].as_str().unwrap();
    // let flag_icons_clone_dest = out_dir.join(slugify(flag_icons_git));

    // let result = update_repository(flag_icons_git, flag_icons_commit, flag_icons_clone_dest);

    // println!("Repository updated: {}", result);

    panic!();

    // let out_res_dir = out_dir.join("res");
    // let out_res_icons_dir = out_dir.join("res/icons");
}
