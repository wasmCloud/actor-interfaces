# wasmcloud Actor Core Interface (Rust)

The wasmcloud actor core interface contains the data types required to respond to a health check request
as sent by the host runtime. In addition, the Rust crate has a convenience macro that allows you to define
a simple `init` function that pre-registers a default "healthy" handler. If you want to supply your own
health check handler, then you can do so with `Handlers::register_health_request`.

Example:

```rust
extern crate wasmcloud_actor_core as actor;

#[actor::init]
pub fn init() {
   // register handlers here
   // default health request handler is already registered
}
```
