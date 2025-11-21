#![allow(unused)]

mod client_protocol;
mod datatypes;
mod protocol_constants;
mod server_protocol;
mod client;
mod server;


pub use datatypes::*;

#[cfg(feature = "client")]
pub use client::generate;

#[cfg(feature = "server")]
pub use server::parse;

#[cfg(all(feature = "client", feature = "server"))]
compile_error!("features 'juno-protocol/client' and 'juno-protocol/server' are mutually exclusive.");
