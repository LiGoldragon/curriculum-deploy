mod catalog;
#[allow(clippy::enum_variant_names)]
#[doc(hidden)]
pub mod generated;
mod role_projection;
mod roles;
mod runtime;

pub use runtime::{CommandLine, Error};
