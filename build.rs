use build_helpers::prelude::*;
use cargo_toml::Manifest;
use serde;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
    foreign_dependencies: ForeignDependenciesDetail,
    gresources: GResourcesDetail,
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest: Manifest<Metadata> = Manifest::from_path_with_metadata("Cargo.toml").unwrap();
    let metadata = manifest.package.unwrap().metadata.unwrap();

    println!("Output directory: {:?}", out_dir);
    // println!("Manifest: {:#?}", manifest);

    for (name, detail) in metadata.foreign_dependencies.into_iter() {
        let updated = detail.update(&out_dir);
        if !updated {
            continue;
        }

    metadata.foreign_dependencies.update_all(&out_dir);

    metadata
        .gresources
        .to_gresources(&out_dir)
        .compile(out_dir.join("assets/compiled.gresource"));
}
