#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # HTTP Server wasmCloud Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the HTTP Server capability provider. Actors using this
//! interface must have the claim `wascc:http_server` in order to have permission to handle requests, and they
//! must have an active, configured binding to an HTTP Server capability provider.
//!
//! The HTTP Server provider is one-way, and only delivers messages to actors. Actors cannot make host calls
//! to this provider.

//! # Example:
//! ```
//! extern crate actor_http_server as http;
//! use wapc_guest::HandlerResult;
//! use http::{Request, Response, Handlers};
//!
//! #[no_mangle]
//! pub fn wapc_init() {
//!     http::Handlers::register_handle_request(hello);
//! }
//!
//! fn hello(_msg: http::Request) -> HandlerResult<http::Response> {
//!     Ok(http::Response::ok())
//! }
//! ```
//!

pub mod generated;

extern crate wapc_guest as guest;
use serde::Serialize;
use std::collections::HashMap;

pub use generated::{deserialize, serialize, Handlers, Request, Response};

impl Response {
    /// Creates a response with a given status code and serializes the given payload as JSON
    pub fn json<T>(payload: T, status_code: u32, status: &str) -> Response
    where
        T: Serialize,
    {
        Response {
            body: serde_json::to_string(&payload).unwrap().into_bytes(),
            header: HashMap::new(),
            status: status.to_string(),
            status_code,
        }
    }

    /// Handy shortcut for creating a 404/Not Found response
    pub fn not_found() -> Response {
        Response {
            status: "Not Found".to_string(),
            status_code: 404,
            ..Default::default()
        }
    }

    /// Useful shortcut for creating a 200/OK response
    pub fn ok() -> Response {
        Response {
            status: "OK".to_string(),
            status_code: 200,
            ..Default::default()
        }
    }

    /// Useful shortcut for creating a 500/Internal Server Error response
    pub fn internal_server_error(msg: &str) -> Response {
        Response {
            status: "Internal Server Error".to_string(),
            status_code: 500,
            body: msg.to_string().as_bytes().into(),
            ..Default::default()
        }
    }

    /// Shortcut for creating a 400/Bad Request response
    pub fn bad_request() -> Response {
        Response {
            status: "Bad Request".to_string(),
            status_code: 400,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Handlers, Request, Response};
    use wapc_guest::HandlerResult;
    #[test]
    fn it_works() {
        Handlers::register_handle_request(hr);
        assert!(true);
    }

    fn hr(_req: Request) -> HandlerResult<Response> {
        Ok(Response::ok())
    }
}
