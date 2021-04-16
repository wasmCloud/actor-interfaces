[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-logging.svg)](https://crates.io/crates/wasmcloud-actor-logging)&nbsp;
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/Logging)
![license](https://img.shields.io/crates/l/wasmcloud-actor-logging.svg)&nbsp;
[![documentation](https://docs.rs/wasmcloud-actor-logging/badge.svg)](https://docs.rs/wasmcloud-actor-logging)
# wasmCloud Logging Actor Interface

This crate provides an abstraction over the `wasmcloud:logging` contract. This
allows actors to use normal log macros (like `info!`, `warn!`, `error!`, etc)
to write logs from within the actor.

Example:
```rust
extern crate wasmcloud_actor_http_server as http;
extern crate wasmcloud_actor_logging as logging;
extern crate wasmcloud_actor_core as actor;
use wapc_guest::HandlerResult;
use http::{Request, Response, Handlers};
use log::{info, warn, error, trace, debug};

#[actor::init]
pub fn init() {
    http::Handlers::register_handle_request(method_logger);
    /// Initialize the logger to enable log macros
    logging::enable_macros();
}

/// Actor must be signed with `wasmcloud:logging` to log messages
fn method_logger(msg: http::Request) -> HandlerResult<http::Response> {
    /// Logs can be directly written via `write_log`
    logging::default().write_log("", "trace", "Coercing Rust String to str");
    
    /// After initialization, logs can be directly written from the actor using macros
    match &*msg.method {
        "GET" => info!("Received a GET request"),
        "POST" => info!("Received a POST request"),
        "PUT" => info!("Received a PUT request"),
        "DELETE" => warn!("Received a DELETE request"),
        req => error!("Received an unsupported HTTP Request: {}", req),
    };
    debug!("Finished matching HTTP method, returning OK");
    Ok(http::Response::ok())
}
```
