use git2::{build::CheckoutBuilder, Oid, Repository};
use std::path::Path;

pub fn update_repository<P: AsRef<Path>>(url: &str, commit: &str, dest_dir: P) -> bool {
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
