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
//! use wasmcloud_actor_core as actor_core;
//!
//! use guest::prelude::*;
//!
//! #[no_mangle]
//! pub fn wapc_init() {
//!     http::Handlers::register_handle_request(handle_http_request);
//!     actor_core::Handlers::register_health_request(health);
//! }
//!
//! fn handle_http_request(_: http::Request) -> HandlerResult<http::Response> {
//!     // Replace `query_db` with `graph::default().query_graph()`
//!     let (name, birth_year): (String, u32) = query_db(
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
//! fn health(_: actor_core::HealthCheckRequest) -> HandlerResult<actor_core::HealthCheckResponse> {
//!   Ok(actor_core::HealthCheckResponse::healthy())   
//! }
//!
//! # fn query_db<T: graph::FromTable>(graph_name: String, query: String) -> ::wapc_guest::HandlerResult<T> {
//! #    T::from_table(&ResultSet {
//! #       statistics: vec![],
//! #       columns: vec![
//! #           Column {
//! #               scalars: Some(vec![Scalar {
//! #                   bool_value: None,
//! #                   double_value: None,
//! #                   int_value: None,
//! #                   string_value: Some("Alice Rider".to_string()),
//! #               }]),
//! #               nodes: Some(vec![]),
//! #               relations: Some(vec![]),
//! #           },
//! #           Column {
//! #               scalars: Some(vec![Scalar {
//! #                   bool_value: None,
//! #                   double_value: None,
//! #                   int_value: Some(1985),
//! #                   string_value: None,
//! #               }]),
//! #               nodes: Some(vec![]),
//! #               relations: Some(vec![]),
//! #           },
//! #       ],
//! #    }).map_err(|e| format!("{}", e).into())
//! # }
//! ```

pub mod generated;
#[allow(unused_imports)]
pub use generated::*;
#[macro_use]
#[cfg(feature = "guest")]
extern crate log;
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
        info!("querying graph");
        let res = self
            .query(graph_name.into(), query.into())
            .map(|res| T::from_table(&res.result_set));
        info!("came back from query");
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
