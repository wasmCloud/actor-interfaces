[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-telnet.svg)](https://crates.io/crates/wasmcloud-actor-telnet)&nbsp;
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/Telnet)
![license](https://img.shields.io/crates/l/wasmcloud-actor-telnet.svg)&nbsp;
[![documentation](https://docs.rs/wasmcloud-actor-telnet/badge.svg)](https://docs.rs/wasmcloud-actor-telnet)
# wasmCloud Telnet Server Actor Interface

This crate provides an abstraction over the `wasmcloud:telnet` contract. This allows
actors to be notified when a new telnet session has started and when text has been
received on a given session. Actors can also emit text to a specific session, which
ultimately correlates to an individual connected socket client.

# Example:
```rust
extern crate wasmcloud_actor_telnet as telnet;
extern crate wasmcloud_actor_core as actor;
use wapc_guest::HandlerResult;

#[actor::init]
pub fn init() {
    telnet::Handlers::register_session_started(session_started);
    telnet::Handlers::register_receive_text(receive_text);
}

fn session_started(session: String) -> HandlerResult<bool> {
   let _ = telnet::default().send_text(session, "Welcome to the Interwebs!\n".to_string());
   Ok(true)
}
fn receive_text(session: String, text: String) -> HandlerResult<bool> {
   let _ = telnet::default().send_text(session, format!("Echo: {}\n", text));
   Ok(true)
}

```
