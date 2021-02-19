[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-core.svg)](https://crates.io/crates/wasmcloud-actor-core)&nbsp;
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/Actor%20Core)
![license](https://img.shields.io/crates/l/wasmcloud-actor-core.svg)&nbsp;
[![documentation](https://docs.rs/wasmcloud-actor-core/badge.svg)](https://docs.rs/wasmcloud-actor-core)

# wasmcloud Core Actor Interface

All actors must respond to the core `HealthCheckRequest` message with either an `Err`
or a `HealthCheckResponse`. The following is an example of what an actor looks like
that only responds to the health check message:

```rust
extern crate wasmcloud_actor_core as actor;
use wapc_guest::HandlerResult;
use actor::{HealthCheckRequest, HealthCheckResponse, Handlers};

 #[actor::init]
 fn init() {
     Handlers::register_health_request(health);
 }

 fn health(_msg: HealthCheckRequest) -> HandlerResult<HealthCheckResponse> {
     Ok(HealthCheckResponse::healthy())
 }
```

The `actor::init` macro defines a health check message responder that always returns `healthy()` by default. If you don't
need to provide your own custom logic for the health check, then your `init` function can be simplified as follows:

```rust
extern crate wasmcloud_actor_core as actor;
use wapc_guest::HandlerResult;

#[actor::init]
fn init() {
    // register your message handlers here
}
```
