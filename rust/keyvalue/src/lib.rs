#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # wasmCloud Key-Value Store Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the key-value capability provider. Actors using this
//! interface must have the claim `wasmcloud:keyvalue` in order to have permission to communicate with the store.
//!
//! The key-value provider is one-way, and only accepts host calls from the actor. This provider does _not_
//! deliver messages to actors.
//!
//! # Example:
//! ```
//! extern crate actor_keyvalue as kv;
//! use wapc_guest::HandlerResult;
//!
//! fn add() -> HandlerResult<()> {
//!   let _ = kv::default().add("test".to_string(), 1)?;    
//!   Ok(())
//! }
//! ```
//!

mod generated;

pub use generated::*;
