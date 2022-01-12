use gio;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use strong_xml::{XmlRead, XmlWrite};
use strum;

#[derive(Debug, Default, Clone, PartialEq, XmlRead, XmlWrite)]
#[xml(tag = "gresources")]
pub struct GResources {
    #[xml(child = "gresource")]
    pub entries: Vec<GResource>,
}

#[derive(Debug, Clone, PartialEq, XmlRead, XmlWrite)]
#[xml(tag = "gresource")]
pub struct GResource {
    #[xml(attr = "prefix")]
    pub prefix: String,
    #[xml(child = "file")]
    pub files: Vec<File>,
}

#[derive(Debug, Default, Clone, PartialEq, XmlRead, XmlWrite)]
#[xml(tag = "file")]
pub struct File {
    #[xml(text)]
    pub path: String,
    #[xml(default, attr = "alias")]
    pub alias: Option<String>,
    #[xml(default, attr = "compressed")]
    pub compressed: Option<bool>,
    #[xml(default, attr = "preprocess")]
    pub preprocess: Option<Preprocess>,
}

#[derive(Debug, Clone, Deserialize, strum::Display, strum::EnumString, PartialEq)]
pub enum Preprocess {
    #[strum(to_string = "xml-stripblanks")]
    #[serde(rename = "xml-stripblanks")]
    XmlStripBlanks,
    #[strum(to_string = "to-pixdata")]
    #[serde(rename = "xml-stripblanks")]
    ToPixData,
}

impl File {
    pub fn new(
        path: String,
        alias: Option<String>,
        compressed: Option<bool>,
        preprocess: Option<Preprocess>,
    ) -> Self {
        Self {
            path,
            alias,
            compressed,
            preprocess,
        }
    }
}

impl GResources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write<P: AsRef<Path>>(&self, dest_file: P) -> std::io::Result<()> {
        fs::write(dest_file, self.to_string().unwrap())
    }

    pub fn compile<P: AsRef<Path>>(&self, dest_file: P) {
        let dest_file = dest_file.as_ref();
        let xml_path = dest_file.with_extension("").with_extension("gresource.xml");

        self.write(&xml_path).unwrap();
        gio::compile_resources("", xml_path.to_str().unwrap(), dest_file.to_str().unwrap());
    }
}

impl FromIterator<GResource> for GResources {
    fn from_iter<I: IntoIterator<Item = GResource>>(iter: I) -> Self {
        Self {
            entries: Vec::from_iter(iter),
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

    pub fn from_iter<I: IntoIterator<Item = File>>(prefix: String, files: I) -> Self {
        Self {
            prefix: prefix,
            files: Vec::from_iter(files),
        }
    }
}
