#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # Core wasmCloud Actor Interface
//!
//! All wasmCloud actors must respond to the core health request message, and all capability
//! providers must be able to receive binding configuration according to the [CapabilityConfiguration](struct.CapabilityConfiguration.html)
//! struct.
//!
//! # Example
//! ```
//! extern crate actor_core as actorcore; // Avoid using the module name `core`
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
#[allow(unused_imports)]
pub use generated::*;

impl HealthCheckResponse {
    pub fn healthy() -> HealthCheckResponse {
        HealthCheckResponse {
            healthy: true,
            message: "".to_string(),
        }
    }
}
