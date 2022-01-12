use crate::foreign_dependencies::ForeignDependency;
use crate::gresources::{File, GResource, Preprocess};
use glob::glob;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

pub type ForeignDepsSet = BTreeMap<String, ForeignDependency>;
pub type GResourceSet = BTreeMap<String, GResourceDetail>;

#[derive(Debug, Deserialize)]
pub struct GResourceFilesDetail {
    pub glob: String,
    pub alias: Option<String>,
    pub compressed: Option<bool>,
    pub preprocess: Option<Preprocess>,
}

pub struct GResourceFilesDetailIter<'a> {
    inner: &'a GResourceFilesDetail,
    glob: glob::Paths,
}

#[derive(Debug, Deserialize)]
pub struct GResourceDetail {
    pub prefix: String,
    pub files: Vec<GResourceFilesDetail>,
}

impl GResourceFilesDetail {
    pub fn to_file_iter<P: AsRef<Path>>(&self, out_dir: P) -> GResourceFilesDetailIter {
        GResourceFilesDetailIter::new(&self, out_dir)
    }
}

impl<'a> GResourceFilesDetailIter<'a> {
    pub fn new<P: AsRef<Path>>(inner: &'a GResourceFilesDetail, out_dir: P) -> Self {
        Self {
            inner,
            glob: glob(out_dir.as_ref().to_str().unwrap()).unwrap(),
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
    pub fn to_gresource<P: AsRef<Path>>(&self, out_dir: P) -> GResource {
        let files = self
            .files
            .iter()
            .flat_map(|detail| detail.to_file_iter(&out_dir));
        GResource::from_iter(self.prefix.clone(), files)
    }
}
