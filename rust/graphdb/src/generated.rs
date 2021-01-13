extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
extern crate wapc_guest as guest;
use guest::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;

pub struct Host {
    binding: String,
}

impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
pub fn default() -> Host {
    Host::default()
}

impl Host {
    pub fn query(&self, query: String, graph_name: String) -> HandlerResult<QueryResponse> {
        let input_args = QueryArgs {
            query: query,
            graph_name: graph_name,
        };
        host_call(
            &self.binding,
            "wasmcloud:graphdb",
            "Query",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<QueryResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn delete(&self, graph_name: String) -> HandlerResult<DeleteResponse> {
        let input_args = DeleteArgs {
            graph_name: graph_name,
        };
        host_call(
            &self.binding,
            "wasmcloud:graphdb",
            "Delete",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<DeleteResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

pub struct Handlers {}

impl Handlers {
    pub fn register_query(f: fn(String, String) -> HandlerResult<QueryResponse>) {
        *QUERY.write().unwrap() = Some(f);
        register_function(&"Query", query_wrapper);
    }
    pub fn register_delete(f: fn(String) -> HandlerResult<DeleteResponse>) {
        *DELETE.write().unwrap() = Some(f);
        register_function(&"Delete", delete_wrapper);
    }
}

lazy_static! {
    static ref QUERY: RwLock<Option<fn(String, String) -> HandlerResult<QueryResponse>>> =
        RwLock::new(None);
    static ref DELETE: RwLock<Option<fn(String) -> HandlerResult<DeleteResponse>>> =
        RwLock::new(None);
}

fn query_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<QueryArgs>(input_payload)?;
    let lock = QUERY.read().unwrap().unwrap();
    let result = lock(input.query, input.graph_name)?;
    Ok(serialize(result)?)
}

fn delete_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<DeleteArgs>(input_payload)?;
    let lock = DELETE.read().unwrap().unwrap();
    let result = lock(input.graph_name)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryArgs {
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "graph_name")]
    pub graph_name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DeleteArgs {
    #[serde(rename = "graph_name")]
    pub graph_name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct QueryResponse {
    #[serde(with = "serde_bytes")]
    #[serde(rename = "data")]
    pub data: Vec<u8>,
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DeleteResponse {
    #[serde(rename = "success")]
    pub success: bool,
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
