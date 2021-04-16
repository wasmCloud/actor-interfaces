extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    /// Register a function to handle an incoming HTTP request from a linked provider
    pub fn register_handle_request(f: fn(Request) -> HandlerResult<Response>) {
        *HANDLE_REQUEST.write().unwrap() = Some(f);
        register_function(&"HandleRequest", handle_request_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static::lazy_static! {
static ref HANDLE_REQUEST: std::sync::RwLock<Option<fn(Request) -> HandlerResult<Response>>> = std::sync::RwLock::new(None);
}

#[cfg(feature = "guest")]
fn handle_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<Request>(input_payload)?;
    let lock = HANDLE_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

/// HTTP Request object
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

/// HTTP Response object
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
