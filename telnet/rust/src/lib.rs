//! # Telnet Server wasmCloud Actor Interface
//!
//! This crate provides an abstraction over the `wasmcloud:telnet` contract. This allows
//! actors to be notified when a new telnet session has started and when text has been
//! received on a given session. Actors can also emit text to a specific session, which
//! ultimately correlates to an individual connected socket client.
//!
//! # Example:
//! ```
//! extern crate actor_telnet as telnet;
//! // extern crate actor_core as actorcore;
//! use wapc_guest::HandlerResult;
//!
//! #[no_mangle]
//! pub fn wapc_init() {
//!     telnet::Handlers::register_session_started(session_started);
//!     telnet::Handlers::register_receive_text(receive_text);
//!//     actorcore::Handlers::register_health_request(health);
//! }
//!
//! fn session_started(session: String) -> HandlerResult<bool> {
//!    let _ = telnet::default().send_text(session, "Welcome to the Interwebs!\n".to_string());
//!    Ok(true)
//! }
//! fn receive_text(session: String, text: String) -> HandlerResult<bool> {
//!    let _ = telnet::default().send_text(session, format!("Echo: {}\n", text));
//!    Ok(true)
//! }
//!
//! ```

#[allow(dead_code)]
mod generated;

pub use generated::*;
