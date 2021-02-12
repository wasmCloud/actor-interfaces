[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-core.svg)](https://crates.io/crates/wasmcloud-actor-core)&nbsp;
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/Actor%20Core)
![license](https://img.shields.io/crates/l/wasmcloud-actor-core.svg)&nbsp;
[![documentation](https://docs.rs/wasmcloud-actor-core/badge.svg)](https://docs.rs/wasmcloud-actor-core)
# wasmCloud Core Actor Interface

All actors must respond to the core `HealthCheckRequest` message with either an `Err`
or a `HealthCheckResponse`. The following is an example of what an actor looks like
that only responds to the health check message:

```rust
 use wapc_guest::HandlerResult;
 use actorcore::{HealthCheckRequest, HealthCheckResponse, Handlers};

 #[no_mangle]
 pub fn wapc_init() {
     Handlers::register_health_request(health);
 }

 fn health(_msg: HealthCheckRequest) -> HandlerResult<HealthCheckResponse> {
     Ok(HealthCheckResponse::healthy())
 }
```
