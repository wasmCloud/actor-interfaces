//! GraphDB wasmCloud Actor Interface
//!
//! This crate provides an abstraction over the `wasmcloud:graphdb` contract. This
//! allows actors to interact with a graph database, such as RedisGraph or Neo4j.

// extern crate wapc_guest as guest;
// use guest::prelude::*;

// #[no_mangle]
// pub fn wapc_init() {
//     Handlers::register_query_graph(query_graph);
//     Handlers::register_delete_graph(delete_graph);
// }

// fn query_graph(_query: String, _graph_name: String) -> HandlerResult<QueryResponse> {
//     Ok(QueryResponse::default()) // TODO: Provide implementation.
// }

// fn delete_graph(_graph_name: String) -> HandlerResult<DeleteResponse> {
//     Ok(DeleteResponse::default()) // TODO: Provide implementation.
// }

pub mod generated;
#[allow(unused_imports)]
use generated::*;

/// The operation to request a query of graph data
pub const OP_QUERY: &str = "QueryGraph";
/// The operation to request the deletion of a graph
pub const OP_DELETE: &str = "DeleteGraph";
