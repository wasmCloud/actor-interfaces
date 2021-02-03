//! # wasmCloud Extras Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the extras capability provider.
//! Every wasmCloud host runtime automatically comes with a built-in extras provider. However,
//! actors using this provider will still need to be signed with the `wasmcloud:extras`
//! capability contract ID.
//!
//! The following functions are supported on the extras `Host` interface:
//!
//! * [request_guid](generated::Host::request_guid)
//! * [request_sequence](generated::Host::request_sequence)
//! * [request_random](generated::Host::request_random)

mod generated;
#[allow(unused_imports)]
pub use generated::*;

/// Constant that can be used by capability providers when handling messages from actors
pub const OP_REQUEST_GUID: &str = "RequestGuid";
/// Constant that can be used by capability providers when handling messages from actors
pub const OP_REQUEST_RANDOM: &str = "RequestRandom";
/// Constant that can be used by capability providers when handling messages from actors
pub const OP_REQUEST_SEQUENCE: &str = "RequestSequence";
