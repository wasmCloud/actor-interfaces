#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # wasmCloud Logging Actor Interface
//!
//!
//! This crate provides an abstraction over the `wasmcloud:logging` contract. This
//! allows actors to use normal log macros (like `info!`, `warn!`, `error!`, etc)
//! to write logs from within the actor.

// extern crate wapc_guest as guest;
// use guest::prelude::*;

// #[no_mangle]
// pub fn wapc_init() {
//     Handlers::register_write_log(write_log);
// }

// fn write_log(_request: WriteLogRequest) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

pub mod generated;
#[allow(unused_imports)]
use generated::*;

// The operation used to request writing a log
pub const OP_LOG: &str = "WriteLog";
