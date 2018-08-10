//! Macros defined by this crate

#[cfg(feature = "errors")]
pub mod error;

#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "options")]
pub mod options;

#[cfg(feature = "status")]
pub mod status;