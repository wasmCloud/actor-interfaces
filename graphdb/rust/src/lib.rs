//! GraphDB wasmCloud Actor Interface
//!
//! This crate provides an abstraction over the `wasmcloud:graphdb` contract. This
//! allows actors to interact with a graph database, such as RedisGraph or Neo4j.
//!
//! Example:
//!
//! ```rust
//! use serde_json::json;
//! extern crate wapc_guest as guest;
//! use wasmcloud_actor_graphdb as graph;
//! use graph::*;
//! use wasmcloud_actor_http_server as http;
//! use wasmcloud_actor_core as actorcore;
//!
//! use guest::prelude::*;
//!
//! #[actorcore::init]
//! pub fn init() {
//!     http::Handlers::register_handle_request(handle_http_request);
//! }
//!
//! fn handle_http_request(_: http::Request) -> HandlerResult<http::Response> {
//!     let (name, birth_year): (String, u32) = graph::default().query_graph(
//!         "MotoGP".to_string(),
//!         "MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, r.birth_year"
//!             .to_string(),
//!     )?;
//!
//!     assert_eq!(name, "Alice Rider".to_string());
//!     assert_eq!(birth_year, 1985);
//!
//!     let result = json!({
//!         "name": name,
//!         "birth_year": birth_year
//!     });
//!
//!     Ok(http::Response::json(result, 200, "OK"))
//! }
//!
//! ```

pub mod generated;
#[allow(unused_imports)]
pub use generated::*;
#[macro_use]
#[cfg(feature = "guest")]
extern crate serde_derive;
#[cfg(feature = "guest")]
mod results;
#[cfg(feature = "guest")]
pub use results::FromTable;
#[cfg(feature = "guest")]
mod conversions;
#[cfg(feature = "guest")]
mod errors;
#[cfg(feature = "guest")]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "guest")]
macro_rules! client_type_error {
    ($($arg:tt)*) => {
        Err($crate::errors::GraphError::ClientTypeError(format!($($arg)*)))
    };
}

#[cfg(feature = "guest")]
impl Host {
    pub fn query_graph<T: FromTable>(
        &self,
        graph_name: String,
        query: String,
    ) -> ::wapc_guest::HandlerResult<T> {
        let res = self
            ._query_graph(graph_name.into(), query.into())
            .map(|res| T::from_table(&res.result_set));
        match res {
            Ok(Ok(l)) => Ok(l),
            _ => Err("Graph conversion error".into()),
        }
    }
}

/// The operation to request a query of graph data
pub const OP_QUERY: &str = "QueryGraph";
/// The operation to request the deletion of a graph
pub const OP_DELETE: &str = "DeleteGraph";
