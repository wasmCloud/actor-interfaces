#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # Core wasmCloud Actor Interface
//!
//! All wasmCloud actors must respond to the core health request message, and all capability
//! providers must be able to receive binding configuration according to the [CapabilityConfiguration](struct.CapabilityConfiguration.html)
//! struct.
//!
//! # Example
//! ```
//! extern crate wasmcloud_actor_core as actorcore; 
//! use wapc_guest::HandlerResult;
//! use actorcore::{HealthCheckRequest, HealthCheckResponse, Handlers};
//!
//! #[no_mangle]
//! pub fn wapc_init() {
//!     Handlers::register_health_request(health);
//! }
//!
//! fn health(_msg: HealthCheckRequest) -> HandlerResult<HealthCheckResponse> {
//!     Ok(HealthCheckResponse::healthy())
//! }
//! ```
//!

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
