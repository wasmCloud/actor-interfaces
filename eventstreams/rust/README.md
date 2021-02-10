# Event Streams wasmCloud Actor Interface

This crate provides an abstraction over the `wasmcloud:eventstreams` contract. This allows
actors to write immutable events to a stream, receive events from a stream,
and query events from a stream.

# Example:
```rust
extern crate wasmcloud_actor_eventstreams as streams;
// extern crate actor_core as actorcore;
use wapc_guest::HandlerResult;
use streams::StreamQuery;
use std::collections::HashMap;

#[no_mangle]
pub fn wapc_init() {
    streams::Handlers::register_deliver_event(deliver_event);
//     actorcore::Handlers::register_health_request(health);
}

fn deliver_event(event: streams::Event) -> HandlerResult<bool> {
   // process event, query streams, or send new events...
   let _evts_so_far = streams::default()
      .query_stream(StreamQuery{
          stream_id: event.stream_id.to_string(),
          range: None,
          count: 0                   
   });
   let _eid = streams::default().write_event("hello_streams".to_string(),
         HashMap::new());
   Ok(true)
}
```

