#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # HTTP Client wasmCloud Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the HTTP client capability provider. Actors using this
//! interface must have the claim `wasmcloud:httpclient` in order to have permission to make outbound HTTP requests,
//! and they must have an active, configured binding to an HTTP Client capability provider.
//!
//! wasmCloud actors without this permission and capability binding will be unable to make outbound HTTP requests.
//!
//! # Example:
//! ```
//! use wapc_guest::HandlerResult;
//! extern crate actor_http_server as httpserver;
//! extern crate actor_http_client as httpclient;
//! extern crate actor_core as core;
//!
//! const API_FQDN: &str = "wasmcloudapi.amazonaws.com"; // Not real
//!
//! #[no_mangle]
//! pub fn wapc_init() {
//!     httpserver::Handlers::register_handle_request(get_proxy);
//!     core::Handlers::register_health_request(health);
//! }
//!
//! fn health(_: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
//!   Ok(core::HealthCheckResponse::healthy())   
//! }
//!
//! /// This function proxys an inbound HTTP request to an external server
//! fn get_proxy(msg: httpserver::Request) -> HandlerResult<httpserver::Response> {
//!     if msg.method == "GET".to_string() {
//!         let req = httpclient::Request {
//!             url: API_FQDN.to_string(),
//!             method: msg.method,
//!             path: msg.path,
//!             query_string: msg.query_string,
//!             header: msg.header,
//!             body: msg.body
//!         };
//!         // Replace `handle_request` with `http::default().request`
//!         let res = handle_request(req)?;
//!         Ok(httpserver::Response {
//!             status_code: res.status_code,
//!             status: res.status,
//!             header: res.header,
//!             body: res.body,
//!         })
//!     } else {
//!         Ok(httpserver::Response::internal_server_error("Only GET requests can be proxied with this actor"))
//!     }
//! }
//!
//! # fn handle_request(req: httpclient::Request) -> HandlerResult<httpclient::Response> {
//! #   Ok(httpclient::Response {
//! #     status: "OK".to_string(),
//! #     status_code: 200,
//! #     ..Default::default()
//! #   })
//! # }

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
#[allow(unused)]
use guest::prelude::*;
mod generated;
pub use generated::*;
#[cfg(feature = "guest")]
use serde::Serialize;
#[cfg(feature = "guest")]
use std::collections::HashMap;
