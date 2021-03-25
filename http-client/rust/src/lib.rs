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
//! extern crate wasmcloud_actor_http_server as httpserver;
//! extern crate wasmcloud_actor_http_client as httpclient;
//! extern crate wasmcloud_actor_core as core;
//!
//! const API_URL: &str = "https://wasmcloudapi.cloud.io/proxy";
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
//!     // Form client request from server request
//!     if msg.method == "GET".to_string() {
//!         // Replace `request` with `httpclient::default().request`
//!         let res = request(msg.method, API_URL.to_string(), msg.header, vec![])?;
//!         // Form server response
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
//! # fn request(method: String, url: String, headers: std::collections::HashMap<String,String>, body: Vec<u8>) -> HandlerResult<httpclient::Response> {
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

pub const OP_REQUEST: &str = "Request";
