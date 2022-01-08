use git2::{build::CheckoutBuilder, Oid, Repository};
use slug::slugify;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml::Value;

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

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("Output directory: {:?}", out_dir);

    let manifest = fs::read_to_string("Cargo.toml")
        .unwrap()
        .parse::<Value>()
        .unwrap();
    let manifest = manifest.as_table().unwrap();
    let metadata = manifest["package"]["metadata"].as_table().unwrap();

    let flag_icons_metadata = metadata["flag-icons"].as_table().unwrap();
    let flag_icons_git = flag_icons_metadata["git"].as_str().unwrap();
    let flag_icons_commit = flag_icons_metadata["commit"].as_str().unwrap();
    let flag_icons_clone_dest = out_dir.join(slugify(flag_icons_git));

    let result = update_repository(flag_icons_git, flag_icons_commit, flag_icons_clone_dest);

    println!("Repository updated: {}", result);

    panic!();

    // let out_res_dir = out_dir.join("res");
    // let out_res_icons_dir = out_dir.join("res/icons");
}
