use build_helpers::prelude::*;
use cargo_toml::Manifest;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
    copy_files: GlobCopySet,
    foreign_dependencies: ForeignDependenciesDetail,
    gresources: GResourcesDetail,
}

fn main() {
    let curr_dir = env::current_dir().unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest: Manifest<Metadata> = Manifest::from_path_with_metadata("Cargo.toml").unwrap();
    let metadata = manifest.package.unwrap().metadata.unwrap();

    println!("Output directory: {:?}", out_dir);
    // println!("Manifest: {:#?}", manifest);

    copy_globs(&metadata.copy_files, &curr_dir, &out_dir);

    metadata.foreign_dependencies.update_all(&out_dir);

    metadata
        .gresources
        .to_gresources(&out_dir)
        .compile(out_dir.join("assets/compiled.gresource"));
}
