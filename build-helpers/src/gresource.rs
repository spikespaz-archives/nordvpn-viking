use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use crate::gresource::*;
    use serde_xml_rs::{from_str, to_string, Deserializer, Serializer};
    use test_case::test_case;
    use xml::reader::{EventReader, ParserConfig};

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
            let config = ParserConfig::new()
                .trim_whitespace(false)
                .whitespace_to_characters(true);

            let event_reader = EventReader::new_with_config(xml.as_bytes(), config);
            let item = File::deserialize(&mut Deserializer::new(event_reader)).unwrap();

            assert_eq!(item, expected);
    }
}
