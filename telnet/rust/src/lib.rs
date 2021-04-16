//! # Telnet Server wasmCloud Actor Interface
//!
//! This crate provides an abstraction over the `wasmcloud:telnet` contract. This allows
//! actors to be notified when a new telnet session has started and when text has been
//! received on a given session. Actors can also emit text to a specific session, which
//! ultimately correlates to an individual connected socket client.
//!
//! # Example:
//! ```
//! extern crate wasmcloud_actor_telnet as telnet;
//! extern crate wasmcloud_actor_core as actor;
//! use telnet::TelnetResult;
//! use wapc_guest::HandlerResult;
//!
//! #[actor::init]
//! pub fn init() {
//!     telnet::Handlers::register_session_started(session_started);
//!     telnet::Handlers::register_receive_text(receive_text);
//! }
//!
//! fn session_started(session: String) -> HandlerResult<TelnetResult> {
//!    let _ = telnet::default().send_text(session, "Welcome to the Interwebs!\n".to_string());
//!    Ok(TelnetResult {
//!      success: true,
//!      error: None,
//!    })
//! }
//! fn receive_text(session: String, text: String) -> HandlerResult<TelnetResult> {
//!    let _ = telnet::default().send_text(session, format!("Echo: {}\n", text));
//!    Ok(TelnetResult {
//!      success: true,
//!      error: None,
//!    })
//! }
//!
//! ```

#[allow(dead_code)]
mod generated;

pub use generated::*;

pub const OP_SEND_TEXT: &str = "SendText";
pub const OP_SESSION_STARTED: &str = "SessionStarted";
pub const OP_RECEIVE_TEXT: &str = "ReceiveText";
