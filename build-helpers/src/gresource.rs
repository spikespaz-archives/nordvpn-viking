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
    use xml::reader::{EventReader, ParserConfig};

    #[test]
    fn test_deserialize_file() {
        {
            let xml = r#"<file>foo/bar/icon.png</file>"#;
            let expected = File {
                path: "foo/bar/icon.png".to_owned(),
                alias: None,
                compressed: false,
                preprocess: None,
            };

            let config = ParserConfig::new()
                .trim_whitespace(false)
                .whitespace_to_characters(true);
            let event_reader = EventReader::new_with_config(xml.as_bytes(), config);
            let item = File::deserialize(&mut Deserializer::new(event_reader)).unwrap();

            println!("{:#?}", item);
            assert_eq!(item, expected);
        }

        {
            let xml = r#"<file alias="image.png">foo/bar/icon.png</file>"#;
            let expected = File {
                path: "foo/bar/icon.png".to_owned(),
                alias: Some("image.png".to_owned()),
                compressed: false,
                preprocess: None,
            };

            let config = ParserConfig::new()
                .trim_whitespace(false)
                .whitespace_to_characters(true);
            let event_reader = EventReader::new_with_config(xml.as_bytes(), config);
            let item = File::deserialize(&mut Deserializer::new(event_reader)).unwrap();

            println!("{:#?}", item);
            assert_eq!(item, expected);
        }

        {
            let xml = r#"<file compressed="true">foo/bar/icon.png</file>"#;
            let expected = File {
                path: "foo/bar/icon.png".to_owned(),
                alias: None,
                compressed: true,
                preprocess: None,
            };

            let config = ParserConfig::new()
                .trim_whitespace(false)
                .whitespace_to_characters(true);
            let event_reader = EventReader::new_with_config(xml.as_bytes(), config);
            let item = File::deserialize(&mut Deserializer::new(event_reader)).unwrap();

            println!("{:#?}", item);
            assert_eq!(item, expected);
        }

        {
            let xml = r#"<file preprocess="to-pixdata">foo/bar/icon.png</file>"#;
            let expected = File {
                path: "foo/bar/icon.png".to_owned(),
                alias: None,
                compressed: false,
                preprocess: Some(Preprocess::ToPixData),
            };

            let config = ParserConfig::new()
                .trim_whitespace(false)
                .whitespace_to_characters(true);
            let event_reader = EventReader::new_with_config(xml.as_bytes(), config);
            let item = File::deserialize(&mut Deserializer::new(event_reader)).unwrap();

            println!("{:#?}", item);
            assert_eq!(item, expected);
        }

        {
            let xml = r#"<file alias="vector.svg" compressed="true" preprocess="xml-stripblanks">foo/bar/icon.svg</file>"#;
            let expected = File {
                path: "foo/bar/icon.svg".to_owned(),
                alias: Some("vector.svg".to_owned()),
                compressed: true,
                preprocess: Some(Preprocess::XmlStripBlanks),
            };

            let config = ParserConfig::new()
                .trim_whitespace(false)
                .whitespace_to_characters(true);
            let event_reader = EventReader::new_with_config(xml.as_bytes(), config);
            let item = File::deserialize(&mut Deserializer::new(event_reader)).unwrap();

            println!("{:#?}", item);
            assert_eq!(item, expected);
        }
    }
}
