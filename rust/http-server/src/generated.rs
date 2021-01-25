extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
extern crate wapc_guest as guest;
use guest::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;

/// Used for making calls from the actor to the host. There are no supported
/// host calls for this capability provider
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

/// Requests a host abstraction for a given binding name
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Requests the default host abstraction
pub fn default() -> Host {
    Host::default()
}

impl Host {
    pub fn handle_request(&self, request: Request) -> HandlerResult<Response> {
        host_call(
            &self.binding,
            "wasmcloud:httpserver",
            "HandleRequest",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<Response>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

/// Used to register message handlers in the actor
pub struct Handlers {}

impl Handlers {
    /// Registers a request handler for the [Request](struct.Request.html) type
    pub fn register_handle_request(f: fn(Request) -> HandlerResult<Response>) {
        *HANDLE_REQUEST.write().unwrap() = Some(f);
        register_function(&"HandleRequest", handle_request_wrapper);
    }
}

lazy_static! {
    static ref HANDLE_REQUEST: RwLock<Option<fn(Request) -> HandlerResult<Response>>> =
        RwLock::new(None);
}

fn handle_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<Request>(input_payload)?;
    let lock = HANDLE_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

/// Represents an HTTP request received by the capability provider and delivered to the actor
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Request {
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "queryString")]
    pub query_string: String,
    #[serde(rename = "header")]
    pub header: std::collections::HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "body")]
    pub body: Vec<u8>,
}

/// The actor responds with an instance of this struct to allow the HTTP server to deliver it
/// to the consuming client
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Response {
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "header")]
    pub header: std::collections::HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "body")]
    pub body: Vec<u8>,
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
