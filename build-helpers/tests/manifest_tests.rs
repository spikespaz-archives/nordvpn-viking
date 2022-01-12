use build_helpers::{
    gresources::{File, Preprocess},
    manifest::*,
};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
// use std::fmt::Debug;
// use test_case::test_case;

static FILES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(file!())
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .join("files")
});

static EXAMPLE_FILES_DETAILS: Lazy<[([File; 3], GResourceFilesDetail); 2]> = Lazy::new(|| {
    [
        (
            [
                File::new(
                    FILES_DIR
                        .join("assets/foo/baz_1.png")
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    Some("images/baz_1.png".to_owned()),
                    Some(true),
                    Some(Preprocess::ToPixData),
                ),
                File::new(
                    FILES_DIR
                        .join("assets/foo/baz_2.png")
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    Some("images/baz_2.png".to_owned()),
                    Some(true),
                    Some(Preprocess::ToPixData),
                ),
                File::new(
                    FILES_DIR
                        .join("assets/foo/baz_3.png")
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    Some("images/baz_3.png".to_owned()),
                    Some(true),
                    Some(Preprocess::ToPixData),
                ),
            ],
            GResourceFilesDetail {
                glob: "foo/baz_*.png".to_owned(),
                alias: Some("images/{}".to_owned()),
                compressed: Some(true),
                preprocess: Some(Preprocess::ToPixData),
            },
        ),
        (
            [
                File::new(
                    FILES_DIR
                        .join("assets/bar/baz_1.png")
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    Some("pictures/baz_1.png".to_owned()),
                    None,
                    None,
                ),
                File::new(
                    FILES_DIR
                        .join("assets/bar/baz_2.png")
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    Some("pictures/baz_2.png".to_owned()),
                    None,
                    None,
                ),
                File::new(
                    FILES_DIR
                        .join("assets/bar/baz_3.png")
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    Some("pictures/baz_3.png".to_owned()),
                    None,
                    None,
                ),
            ],
            GResourceFilesDetail {
                glob: "bar/baz_*.png".to_owned(),
                alias: Some("pictures/{}".to_owned()),
                compressed: None,
                preprocess: None,
            },
        ),
    ]
});

#[test]
fn print_files_dir() {
    println!("{}", FILES_DIR.to_str().unwrap());
}
