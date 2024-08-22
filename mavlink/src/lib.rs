#![cfg_attr(not(feature = "std"), no_std)]
// include generate definitions
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub use mavlink_core::*;

#[cfg(feature = "emit-extensions")]
#[allow(unused_imports)]
pub(crate) use mavlink_core::utils::RustDefault;
