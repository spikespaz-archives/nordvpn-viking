use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "gresources")]
pub struct GResources {
    #[serde(rename = "$value")]
    pub entries: Vec<GResource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "gresource")]
pub struct GResource {
    pub prefix: String,
    #[serde(rename = "$value")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
pub enum Entry {
    File(File),
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "file")]
pub struct File {
    #[serde(rename = "$value")]
    pub path: String,
    #[serde(default)]
    pub alias: Option<String>,
    #[serde(default)]
    pub compressed: bool,
    #[serde(default)]
    pub preprocess: Option<Preprocess>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "preprocess")]
pub enum Preprocess {
    #[serde(rename = "xml-stripblanks")]
    XmlStripBlanks,
    #[serde(rename = "to-pixdata")]
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
            entries: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gresource::*;
    use quick_xml;
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
        let file: File = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(file, expected);
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
        let xml = quick_xml::se::to_string(&file).unwrap();
        assert_eq!(xml, expected);
    }
}
