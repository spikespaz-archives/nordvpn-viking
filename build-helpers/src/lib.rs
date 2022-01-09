pub mod foreign_dependencies;
pub mod git;

pub mod prelude {
    pub use crate::foreign_dependencies::*;
    pub use cargo_toml::Manifest;
}
