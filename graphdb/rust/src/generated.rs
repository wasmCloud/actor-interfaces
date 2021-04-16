extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
use lazy_static::lazy_static;
#[cfg(feature = "guest")]
use std::sync::RwLock;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    /// Execute a query on a given graph
    pub(crate) fn _query_graph(
        &self,
        graph_name: String,
        query: String,
    ) -> HandlerResult<QueryResponse> {
        let input_args = QueryGraphArgs { graph_name, query };
        host_call(
            &self.binding,
            "wasmcloud:graphdb",
            "QueryGraph",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<QueryResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Delete a graph from the database
    pub fn delete_graph(&self, graph_name: String) -> HandlerResult<DeleteResponse> {
        let input_args = DeleteGraphArgs { graph_name };
        host_call(
            &self.binding,
            "wasmcloud:graphdb",
            "DeleteGraph",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<DeleteResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryGraphArgs {
    #[serde(rename = "graphName")]
    pub graph_name: String,
    #[serde(rename = "query")]
    pub query: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DeleteGraphArgs {
    #[serde(rename = "graphName")]
    pub graph_name: String,
}

/// Result from a graph query
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryResponse {
    #[serde(rename = "resultSet")]
    pub result_set: ResultSet,
}

/// Indicates the success of a delete operation
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DeleteResponse {
    #[serde(rename = "success")]
    pub success: bool,
}

/// Result from a graph query, contains a list of columns which were returned from
/// the query and statistics about query time
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ResultSet {
    #[serde(rename = "columns")]
    pub columns: Vec<Column>,
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
}

/// A single entity in a graph database, which may be a scalar, node, or relation
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Column {
    #[serde(rename = "scalars")]
    pub scalars: Option<Vec<Scalar>>,
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<Node>>,
    #[serde(rename = "relations")]
    pub relations: Option<Vec<Relation>>,
}

/// Represents a scalar value, all fields should be examined to determine the value
/// contained in the scalar
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Scalar {
    #[serde(rename = "boolValue")]
    pub bool_value: Option<bool>,
    #[serde(rename = "intValue")]
    pub int_value: Option<i64>,
    #[serde(rename = "doubleValue")]
    pub double_value: Option<f64>,
    #[serde(rename = "stringValue")]
    pub string_value: Option<String>,
}

/// A node in a graph database, comprised of a list of labels and a map of
/// properties
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Node {
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, Scalar>,
}

/// A relationship between exactly two nodes
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Relation {
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, Scalar>,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
