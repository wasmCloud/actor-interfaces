#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # Core wasmCloud Actor Interface
//!
//! This crate contains the data types required by all actors, namely the health check request and
//! health check response, and [CapabilityConfiguration](struct.CapabilityConfiguration.html), a struct used
//! by capability providers to receive link data for an actor.
//!
//! If you use the `init` macro, then a default health check handler will be created for you, as shown in
//! this example. If you want to provide your own custom health check handler, then simply call
//! `Handlers::register_health_check` with your handler function.
//!
//! # Example
//! ```
//! extern crate wasmcloud_actor_core as actor;
//!
//! #[actor::init]
//! pub fn init() {
//!     // register handlers here
//! }
//! ```
//!
//! # Caveat
//! Your `init` function is called by the wasmcloud host runtime when the actor is first loaded into the host. This function
//! is only ever called once, and _is called before any provider linking takes place_. In other words, code written inside this
//! function cannot communicate with capability providers.
//!
//! Also, in keeping with the notion of _stateless_ actors, avoid using this function to initialize or create global state.

mod generated;
pub use generated::{
    deserialize, serialize, CapabilityConfiguration, Handlers, HealthCheckRequest,
    HealthCheckResponse,
};

#[cfg(feature = "guest")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "guest")]
/// Performs an actor-to-actor call, with the target actor identified by a reference string. This
/// reference can be an OCI image URL, a 56-character public key (subject), or, if one is defined,
/// a developer-friendly call alias
pub fn call_actor<'de, T: Serialize, U: Deserialize<'de>>(
    actor_ref: &str,
    operation: &str,
    msg: &T,
) -> wapc_guest::HandlerResult<U> {
    let res = wapc_guest::host_call("default", actor_ref, operation, &generated::serialize(msg)?)?;
    let res = generated::deserialize(&res)?;
    Ok(res)
}

impl HealthCheckResponse {
    pub fn healthy() -> HealthCheckResponse {
        HealthCheckResponse {
            healthy: true,
            message: "".to_string(),
        }
    }
}

#[cfg(feature = "guest")]
pub use wasmcloud_actor_core_derive::init;
