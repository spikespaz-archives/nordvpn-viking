use crate::foreign_dependencies::ForeignDependency;
use crate::gresources::Preprocess;
use serde::Deserialize;
use std::collections::BTreeMap;

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

