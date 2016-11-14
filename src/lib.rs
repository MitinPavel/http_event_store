#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#[macro_use]
extern crate hyper;

extern crate uuid;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_macros")]
include!("lib.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

pub mod expected_version;
pub mod event;
pub mod client;
pub mod error;
mod connection;
mod macros;
mod api;

pub use self::error::HesError;
