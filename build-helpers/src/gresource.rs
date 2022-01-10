use strong_xml::{XmlRead, XmlWrite};
use strum;

#[derive(Debug)]
pub struct GResources {
    pub entries: Vec<GResource>,
}

#[derive(Debug)]
pub struct GResource {
    pub prefix: String,
    pub files: Vec<File>,
}

#[derive(Debug, Default, PartialEq, XmlRead, XmlWrite)]
#[xml(tag = "file")]
pub struct File {
    #[xml(text)]
    pub path: String,
    #[xml(default, attr = "alias")]
    pub alias: Option<String>,
    #[xml(default, attr = "compressed")]
    pub compressed: bool,
    #[xml(default, attr = "preprocess")]
    pub preprocess: Option<Preprocess>,
}

#[derive(Debug, strum::Display, strum::EnumString, PartialEq)]
pub enum Preprocess {
    #[strum(to_string = "xml-stripblanks")]
    XmlStripBlanks,
    #[strum(to_string = "to-pixdata")]
    ToPixData,
}

impl GResources {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl GResource {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            files: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gresource::*;
    use once_cell::sync::Lazy;
    use test_case::test_case;

    static EXAMPLE_FILES: Lazy<[(&'static str, File); 6]> = Lazy::new(|| {
        [
            (
                r#"<file>foo/bar/baz_1.png</file>"#,
                File {
                    path: "foo/bar/baz_1.png".to_owned(),
                    alias: None,
                    compressed: false,
                    preprocess: None,
                },
            ),
            (
                r#"<file alias="image.png">foo/bar/baz_2.png</file>"#,
                File {
                    path: "foo/bar/baz_2.png".to_owned(),
                    alias: Some("image.png".to_owned()),
                    compressed: false,
                    preprocess: None,
                },
            ),
            (
                r#"<file compressed="true">foo/bar/baz_3.png</file>"#,
                File {
                    path: "foo/bar/baz_3.png".to_owned(),
                    alias: None,
                    compressed: true,
                    preprocess: None,
                },
            ),
            (
                r#"<file preprocess="to-pixdata">foo/bar/baz_4.png</file>"#,
                File {
                    path: "foo/bar/baz_4.png".to_owned(),
                    alias: None,
                    compressed: false,
                    preprocess: Some(Preprocess::ToPixData),
                },
            ),
            (
                r#"<file alias="image.png" compressed="true" preprocess="to-pixdata">foo/bar/baz_5.png</file>"#,
                File {
                    path: "foo/bar/baz_5.png".to_owned(),
                    alias: Some("image.png".to_owned()),
                    compressed: true,
                    preprocess: Some(Preprocess::ToPixData),
                },
            ),
            (
                r#"<file alias="icon.svg" compressed="true" preprocess="xml-stripblanks">foo/bar/baz_6.svg</file>"#,
                File {
                    path: "foo/bar/baz_6.svg".to_owned(),
                    alias: Some("icon.svg".to_owned()),
                    compressed: true,
                    preprocess: Some(Preprocess::XmlStripBlanks),
                },
            ),
        ]
    });

    #[test_case(EXAMPLE_FILES[0].0, &EXAMPLE_FILES[0].1 ; "test deserialize file 1")]
    #[test_case(EXAMPLE_FILES[1].0, &EXAMPLE_FILES[1].1 ; "test deserialize file 2")]
    #[test_case(EXAMPLE_FILES[2].0, &EXAMPLE_FILES[2].1 ; "test deserialize file 3")]
    #[test_case(EXAMPLE_FILES[3].0, &EXAMPLE_FILES[3].1 ; "test deserialize file 4")]
    #[test_case(EXAMPLE_FILES[4].0, &EXAMPLE_FILES[4].1 ; "test deserialize file 5")]
    #[test_case(EXAMPLE_FILES[5].0, &EXAMPLE_FILES[5].1 ; "test deserialize file 6")]
    fn test_deserialize_file(xml: &str, expected: &File) {
        let result = File::from_str(xml).unwrap();
        assert_eq!(expected, &result);
    }

    // fn test_serialize_file(file: &File, expected: &str) {
    //     let xml = file.to_string().unwrap();
    //     assert_eq!(expected, xml);
    // }
}
