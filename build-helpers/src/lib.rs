pub mod foreign_dependencies;
pub mod git;
pub mod gresource;

pub mod prelude {
    pub use crate::foreign_dependencies::*;
    pub use crate::gresource::*;
    pub use cargo_toml::Manifest;
}
