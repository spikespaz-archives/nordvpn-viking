pub mod common;
pub mod fdependencies;
pub mod gresources;

pub mod manifest {
    pub use crate::fdependencies::manifest::*;
    pub use crate::gresources::manifest::*;
}

pub mod prelude {
    pub use crate::common::*;
    pub use crate::fdependencies::*;
    pub use crate::gresources::*;
    pub use crate::manifest::*;
}
