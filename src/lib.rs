#[allow(clippy::enum_variant_names)]
#[doc(hidden)]
pub mod generated;
mod generated_ext;
mod roles;
mod runtime;

pub use runtime::{CommandLine, Error};
