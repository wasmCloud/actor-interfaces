#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # wasmCloud Logging Actor Interface
//!
//! This crate provides an abstraction over the `wasmcloud:logging` contract. This
//! allows actors to use normal log macros (like `info!`, `warn!`, `error!`, etc)
//! to write logs from within the actor.
//!
//! Example:
//! ```rust
//! extern crate actor_http_server as http;
//! use wapc_guest::HandlerResult;
//! use http::{Request, Response, Handlers};
//! use log::{info, warn, error, trace, debug};
//!
//! #[no_mangle]
//! pub fn wapc_init() {
//!     http::Handlers::register_handle_request(method_logger);
//! }
//!
//! /// If signed with `wasmcloud:logging`, log macros will emit `WriteLog` operations
//! /// to be handled by an appropriate capability provider.
//! fn method_logger(msg: http::Request) -> HandlerResult<http::Response> {
//!     trace!("Coercing Rust String to str");
//!     match &*msg.method {
//!         "GET" => info!("Received a GET request"),
//!         "POST" => info!("Received a POST request"),
//!         "PUT" => info!("Received a PUT request"),
//!         "DELETE" => warn!("Received a DELETE request"),
//!         req => error!("Received an unsupported HTTP Request: {}", req),
//!     };
//!     debug!("Finished matching HTTP method, returning OK");
//!     Ok(http::Response::ok())
//! }
//! ```

mod generated;
#[allow(unused_imports)]
pub use generated::*;

// The operation used to request writing a log
pub const OP_LOG: &str = "WriteLog";
