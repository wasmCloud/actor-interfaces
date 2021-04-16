[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-keyvalue.svg)](https://crates.io/crates/wasmcloud-actor-keyvalue)&nbsp;
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/KeyValue)
![license](https://img.shields.io/crates/l/wasmcloud-actor-keyvalue.svg)&nbsp;
[![documentation](https://docs.rs/wasmcloud-actor-keyvalue/badge.svg)](https://docs.rs/wasmcloud-actor-keyvalue)
# wasmCloud Key Value Store Actor Interface

This crate provides an interface for actors to use to communicate with a key-value
store capability provider. Actors using this interface must have the `wasmcloud:keyvalue` capability
permission.

This crate is _one-way_, and only supports actors making calls to the host. The capability provider does not deliver messages to actors (e.g. actors cannot subscribe to store change events).

The following is an example usage:

```rust
#[macro_use]
extern crate serde_json;
extern crate wasmcloud_actor_http_server as http;
extern crate wasmcloud_actor_keyvalue as kv;
extern crate wasmcloud_actor_core as actor;

use http::{self, Request, Response};
use wascap_guest::HandlerResult;

#[macro_use]
extern crate serde_json;

#[actor::init]
pub fn init() {
    http::Handlers::register_handle_request(increment_counter);
}

fn increment_counter(msg: Request) -> HandlerResult<Response> {
    let key = msg.path.replace('/', ":");
    let resp = kv::default().add(key.to_string(), 1)?;

    let result = json!({"counter": resp.value });
    Ok(Response::json(result, 200, "OK")?)
}
```
