use serde::Deserialize;
use std::{fs, path::Path, process::Command};
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

        let status = Command::new("glib-compile-resources")
            .arg("--target")
            .arg(dest_file)
            .arg(xml_path)
            .status()
            .unwrap();

        assert!(
            status.success(),
            "glib-compile-resources failed with exit status {}",
            status
        );
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

pub mod manifest {
    use crate::gresources::{File, GResource, GResources, Preprocess};
    use glob::glob;
    use serde::Deserialize;
    use std::{collections::BTreeMap, path::Path};

    #[derive(Debug, Deserialize)]
    pub struct GResourceFilesDetail {
        pub glob: String,
        pub alias: Option<String>,
        pub compressed: Option<bool>,
        pub preprocess: Option<Preprocess>,
    }

    #[derive(Debug, Deserialize)]
    pub struct GResourceDetail {
        pub prefix: String,
        pub files: Vec<GResourceFilesDetail>,
    }

    pub struct GResourceFilesDetailIter<'a> {
        inner: &'a GResourceFilesDetail,
        glob: glob::Paths,
    }

    #[derive(Debug, Deserialize)]
    pub struct GResourcesDetail(BTreeMap<String, GResourceDetail>);

    impl GResourceFilesDetail {
        pub fn expand<P: AsRef<Path>>(&self, src_dir: P) -> GResourceFilesDetailIter {
            GResourceFilesDetailIter::new(&self, src_dir)
        }
    }

    impl<'a> GResourceFilesDetailIter<'a> {
        pub fn new<P: AsRef<Path>>(inner: &'a GResourceFilesDetail, src_dir: P) -> Self {
            Self {
                inner,
                glob: glob(src_dir.as_ref().join(&inner.glob).to_str().unwrap()).unwrap(),
            }
        }
    }

    impl Iterator for GResourceFilesDetailIter<'_> {
        type Item = File;

        fn next(&mut self) -> Option<Self::Item> {
            let file_path = self.glob.next()?.unwrap();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let alias = self
                .inner
                .alias
                .clone()
                .map(|alias| alias.replace("{}", file_name));

            Some(File::new(
                file_path.to_str().to_owned().unwrap().to_string(),
                alias.map(|alias| alias.clone()),
                self.inner.compressed.clone(),
                self.inner.preprocess.clone(),
            ))
        }
    }

    impl GResourceDetail {
        pub fn to_gresource<P: AsRef<Path>>(&self, src_dir: P) -> GResource {
            let files = self.files.iter().flat_map(|detail| detail.expand(&src_dir));
            GResource::from_iter(self.prefix.clone(), files)
        }
    }

    impl GResourcesDetail {
        pub fn to_gresources<P: AsRef<Path>>(&self, src_dir: P) -> GResources {
            GResources::from_iter(
                self.0
                    .iter()
                    .map(|(_, detail)| detail.to_gresource(&src_dir)),
            )
        }
    }
}
