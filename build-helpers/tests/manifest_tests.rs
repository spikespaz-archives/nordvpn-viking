use build_helpers::{
    gresources::{File, Preprocess},
    manifest::*,
};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use test_case::test_case;

static FILES_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("tests/files"));
static TEMP_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("target/tmp"));

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
                glob: "assets/foo/baz_*.png".to_owned(),
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
                glob: "assets/bar/baz_*.png".to_owned(),
                alias: Some("pictures/{}".to_owned()),
                compressed: None,
                preprocess: None,
            },
        ),
    ]
});

#[test_case(&EXAMPLE_FILES_DETAILS[0].1, &EXAMPLE_FILES_DETAILS[0].0 ; "test iter files details 1")]
#[test_case(&EXAMPLE_FILES_DETAILS[1].1, &EXAMPLE_FILES_DETAILS[1].0 ; "test iter files details 2")]
fn test_expand_files(detail: &GResourceFilesDetail, expected: &[File]) {
    let gresources = detail.expand(&*FILES_DIR);

    for (result, expected) in Iterator::zip(gresources, expected.into_iter()) {
        assert_eq!(result, *expected);
    }
}
