#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # Messaging wasmCloud Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the Messaging capability provider. Actors using this
//! interface must have the claim `wasmcloud:messaging` in order to have permission to handle messages, publish
//! and perform request-response actions. They also must have an active, configured binding to a Messaging capability provider.
//!
//! # Example:
//! ```rust
//! extern crate wasmcloud_actor_messaging as messaging;
//! extern crate wasmcloud_actor_core as actor;
//! extern crate wapc_guest as guest;
//! use guest::prelude::*;
//!
//! #[actor::init]
//! pub fn init() {
//!     messaging::Handlers::register_handle_message(handle_message);
//! }
//!
//! /// Reply to a "ping" message with "pong"
//! fn handle_message(message: messaging::BrokerMessage) -> HandlerResult<()> {
//!     if String::from_utf8(message.body)? == "ping".to_string() {
//!         messaging::default().publish(message.reply_to, "".to_string(), "pong".to_string().into_bytes())?;
//!     }
//!     Ok(())
//! }
//!
//! ```

mod generated;
pub use generated::*;

pub const OP_HANDLE_MESSAGE: &str = "HandleMessage";
