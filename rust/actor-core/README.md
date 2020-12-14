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
