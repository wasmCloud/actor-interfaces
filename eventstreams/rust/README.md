[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-eventstreams.svg)](https://crates.io/crates/wasmcloud-actor-eventstreams)
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/Event%20Streams)
![license](https://img.shields.io/crates/l/wasmcloud-actor-eventstreams.svg)
[![documentation](https://docs.rs/wasmcloud-actor-eventstreams/badge.svg)](https://docs.rs/wasmcloud-actor-eventstreams)

# wasmCloud Event Streams Actor Interface

This crate provides an abstraction over the `wasmcloud:eventstreams` contract. This allows
actors to write immutable events to a stream, receive events from a stream,
and query events from a stream.

# Example

```rust
#[macro_use]
extern crate wasmcloud_actor_core as actor;
extern crate wasmcloud_actor_eventstreams as streams;
extern crate wasmcloud_actor_http_server as http;

use wapc_guest::HandlerResult;
use streams::*;
use std::collections::HashMap;

#[actor::init]
fn init() {
   http::Handlers::register_handle_request(handle_request);
}

fn handle_request(_req: http::Request) -> HandlerResult<http::Response> {
   // process event, query streams, or send new events...
   let _evts_so_far = streams::default()
      .query_stream(StreamQuery{
          stream_id: "hello_stream".to_string(),
          range: None,
          count: 0                   
   });
   let ack = streams::default().write_event("hello_stream".to_string(),
         HashMap::new())?;
   Ok(http::Response::ok())
}
```
