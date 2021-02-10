# wasmCloud Extras Actor Interface
//!
This crate provides wasmCloud actors with an interface to the extras capability provider.
Every wasmCloud host runtime automatically comes with a built-in extras provider. However,
actors using this provider will still need to be signed with the `wasmcloud:extras`
capability contract ID.
//!
The following functions are supported on the extras `Host` interface:
//!
* [request_guid](generated::Host::request_guid)
* [request_sequence](generated::Host::request_sequence)
* [request_random](generated::Host::request_random)
//!
Example:
//!
```rust
extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_core as core;
use wasmcloud_actor_extras as extras;
use wasmcloud_actor_http_server as http;
use serde_json::json;
use log::{error, info};
//!
#[no_mangle]
pub fn wapc_init() {
    http::Handlers::register_handle_request(generate_guid);
    core::Handlers::register_health_request(health);
}
//!
/// Generate a Guid and return it in a JSON envelope
fn generate_guid(_req: http::Request) -> HandlerResult<http::Response> {
  let guid = get_guid()?      // Replace this with `extras::default().request_guid()?`
                .unwrap_or("unknown-guid".to_string());
//!
  let result = json!({"guid": guid });
  Ok(http::Response::json(&result, 200, "OK"))
//!
}
//!
fn health(_: core::HealthCheckRequest) -> HandlerResult<core::HealthCheckResponse> {
  Ok(core::HealthCheckResponse::healthy())   
}
//!
# fn get_guid() -> HandlerResult<Option<String>> {
#   Ok(Some("test".to_string()))
# }
//!
```

