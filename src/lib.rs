#[allow(clippy::enum_variant_names)]
#[doc(hidden)]
pub mod generated;
mod roles;
mod runtime;

pub use runtime::{CommandLine, Error};
