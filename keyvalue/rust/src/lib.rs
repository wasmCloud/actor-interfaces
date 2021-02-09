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
//! extern crate wasmcloud_actor_keyvalue as kv;
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

pub const OP_ADD: &str = "Add";
pub const OP_GET: &str = "Get";
pub const OP_SET: &str = "Set";
pub const OP_DEL: &str = "Del";
pub const OP_CLEAR: &str = "Clear";
pub const OP_RANGE: &str = "Range";
pub const OP_PUSH: &str = "Push";
pub const OP_LIST_DEL: &str = "ListItemDelete";

pub const OP_SET_ADD: &str = "SetAdd";
pub const OP_SET_REMOVE: &str = "SetRemove";
pub const OP_SET_UNION: &str = "SetUnion";
pub const OP_SET_INTERSECT: &str = "SetIntersection";
pub const OP_SET_QUERY: &str = "SetQuery";
pub const OP_KEY_EXISTS: &str = "KeyExists";
