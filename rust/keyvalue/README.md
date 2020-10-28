# waSCC Key Value Store Interface

This crate provides an interface for actors to use to communicate with a key-value
store capability provider. Actors using this interface must have the `wascc:key_value` capability
permission.

This crate is _one-way_, and only supports actors making calls to the host. The capability provider
does not deliver messages to actors.

The following is an example usage:

```rust
#[macro_use]
extern crate serde_json;
extern crate actor_http_server as http;
extern crate actor_keyvalue as kv;
use http::{self, Request, Response};
use wascap_guest::HandlerResult;

#[no_mangle]
pub fn wapc_init() {
    http::Handlers::register_handle_request(increment_counter);
}

#[macro_use]
extern crate serde_json;

fn increment_counter(msg: Request) -> HandlerResult<Response> {
    let key = msg.path.replace('/', ":");
    let resp = kv::default().add(key.to_string(), 1)?;

    let result = json!({"counter": resp.value });
    Ok(Response::json(result, 200, "OK")?)
}
```
