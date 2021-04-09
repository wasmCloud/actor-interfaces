[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-http-client.svg)](https://crates.io/crates/wasmcloud-actor-http-client)&nbsp;
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/HTTP-Client)
![license](https://img.shields.io/crates/l/wasmcloud-actor-http-client.svg)&nbsp;
[![documentation](https://docs.rs/wasmcloud-actor-http-client/badge.svg)](https://docs.rs/wasmcloud-actor-http-client)
# wasmCloud HTTP Client Actor Interface

This crate provides wasmCloud actors with an interface to the HTTP client capability provider. Actors using this
interface must have the claim `wasmcloud:httpclient` in order to have permission to make outbound HTTP requests,
and they must have an active, configured binding to an HTTP Client capability provider.

wasmCloud actors without this permission and capability binding will be unable to make outbound HTTP requests.

# Example:
```rust
use wapc_guest::HandlerResult;
extern crate wasmcloud_actor_http_server as httpserver;
extern crate wasmcloud_actor_http_client as httpclient;
extern crate wasmcloud_actor_core as actorcore;

const API_URL: &str = "https://wasmcloudapi.cloud.io/proxy";

#[actorcore::init]
pub fn init() {
    httpserver::Handlers::register_handle_request(get_proxy);
}

/// This function proxys an inbound HTTP request to an external server
fn get_proxy(msg: httpserver::Request) -> HandlerResult<httpserver::Response> {
    // Form client request from server request
    if msg.method == "GET".to_string() {
        // Replace `request` with `httpclient::default().request`
        let res = request(msg.method, API_URL.to_string(), msg.header, vec![])?;
        // Form server response
        Ok(httpserver::Response {
            status_code: res.status_code,
            status: res.status,
            header: res.header,
            body: res.body,
        })
    } else {
        Ok(httpserver::Response::internal_server_error("Only GET requests can be proxied with this actor"))
    }
}
```

