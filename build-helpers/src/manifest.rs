use crate::foreign_dependencies::ForeignDependency;
use crate::gresources::{File, GResource, GResources, Preprocess};
use glob::glob;
use serde::Deserialize;
use std::{collections::BTreeMap, path::Path};

#[derive(Debug, Deserialize)]
pub struct ForeignDependenciesDetail(BTreeMap<String, ForeignDependency>);

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

impl ForeignDependenciesDetail {
    pub fn update_all<P: AsRef<Path>>(&self, out_dir: P) {
        for (_, detail) in self.0.iter() {
            let updated = detail.update(&out_dir);

            if !updated {
                continue;
            }
        }
    }
}

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
