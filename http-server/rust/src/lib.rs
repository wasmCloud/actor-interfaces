#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # HTTP Server wasmCloud Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the HTTP Server capability provider. Actors using this
//! interface must have the claim `wasmcloud:httpserver` in order to have permission to handle requests, and they
//! must have an active, configured binding to an HTTP Server capability provider.
//!
//! The HTTP Server provider is one-way, and only delivers messages to actors. Actors cannot make host calls
//! to this provider. To make outbound http requests, actors will need to use a `wasmcloud:httpclient` provider.
//!
//! # Example:
//! ```
//! use wasmcloud_actor_http_server as http;
//! use wasmcloud_actor_core as actor;
//! use wapc_guest::HandlerResult;
//! use http::{Request, Response, Handlers, Method};
//!
//! #[actor::init]
//! fn init() {
//!     http::Handlers::register_handle_request(req_handler);
//! }
//!
//! fn req_handler(req: http::Request) -> HandlerResult<http::Response> {
//!     let method = req.method();
//!     let segments = req.path_segments();
//!
//!     match (method, &*segments)  {
//!         (Method::Get, &["v0", "users", id]) => get_user(id),
//!         (Method::Put, &["v1", "users", id]) => update_user(id, &req.body),
//!         _ => Ok(http::Response::not_found())
//!     }
//! }
//!
//! fn get_user(id: &str) -> HandlerResult<http::Response> {
//!     Ok(http::Response::ok())
//! }
//! fn update_user(id: &str, body: &[u8]) -> HandlerResult<http::Response> {
//!     Ok(http::Response::ok())
//! }
//! ```

pub mod generated;
mod route;
use serde::Serialize;
use std::collections::HashMap;

pub use route::Method;

#[cfg(feature = "guest")]
pub use generated::Handlers;
pub use generated::{deserialize, serialize, Request, Response};

use std::str::FromStr;

impl Request {
    pub fn path_segments(&self) -> Vec<&str> {
        self.path.split('/').skip(1).collect::<Vec<_>>()
    }

    pub fn method(&self) -> Method {
        Method::from_str(&self.method).unwrap()
    }
}

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
#[cfg(feature = "guest")]
mod test {
    extern crate wapc_guest;
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

#[cfg(test)]
#[cfg(feature = "guest")]
mod path_segments {
    extern crate wapc_guest;
    use crate::Request;
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let request = test_request("");
        let segments = request.path_segments();
        assert_eq!(segments.clone().len(), 0);
    }

    #[test]
    fn single_slash() {
        let request = test_request("/");
        let segments = request.path_segments();
        assert_eq!(segments.clone().len(), 0);
    }

    #[test]
    fn single() {
        let request = test_request("/foo");
        let segments = request.path_segments();
        assert_eq!(segments.clone().len(), 1);
    }

    #[test]
    fn trailing_slash() {
        let request = test_request("/foo/");
        let segments = request.path_segments();
        assert_eq!(segments.clone().len(), 1);
    }

    fn test_request(path: &str) -> Request {
        Request {
            method: "GET".to_string(),
            body: Vec::new(),
            header: HashMap::new(),
            path: path.to_string(),
            query_string: "".to_string(),
        }
    }

}
