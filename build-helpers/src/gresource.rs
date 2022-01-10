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
    use test_case::test_case;

    #[test_case(
        r#"<file>foo/bar/icon.png</file>"#,
        File {
            path: "foo/bar/icon.png".to_owned(),
            alias: None,
            compressed: false,
            preprocess: None,
        }
    )]
    #[test_case(
        r#"<file alias="icon.png">foo/bar/icon.png</file>"#,
        File {
            path: "foo/bar/icon.png".to_owned(),
            alias: Some("icon.png".to_owned()),
            compressed: false,
            preprocess: None,
        }
    )]
    #[test_case(
        r#"<file preprocess="to-pixdata">foo/bar/icon.png</file>"#,
        File {
            path: "foo/bar/icon.png".to_owned(),
            alias: None,
            compressed: false,
            preprocess: Some(Preprocess::ToPixData),
        }
    )]
    #[test_case(
        r#"<file preprocess="xml-stripblanks">foo/bar/vector.svg</file>"#,
        File {
            path: "foo/bar/vector.svg".to_owned(),
            alias: None,
            compressed: false,
            preprocess: Some(Preprocess::XmlStripBlanks),
        }
    )]
    #[test_case(
        r#"<file compressed="true" preprocess="to-pixdata">foo/bar/icon.png</file>"#,
        File {
            path: "foo/bar/icon.png".to_owned(),
            alias: None,
            compressed: true,
            preprocess: Some(Preprocess::ToPixData),
        }
    )]
    fn test_deserialize_file(xml: &str, expected: File) {
        let file = File::from_str(xml).unwrap();
        assert_eq!(expected, file);
    }

    #[test_case(
        File {
            path: "foo/bar/icon.png".to_owned(),
            alias: None,
            compressed: true,
            preprocess: Some(Preprocess::ToPixData),
        },
        r#"<file compressed="true" preprocess="to-pixdata">foo/bar/icon.png</file>"#
    )]
    fn test_serialize_file(file: File, expected: &str) {
        let xml = file.to_string().unwrap();
        assert_eq!(expected, xml);
    }
}
