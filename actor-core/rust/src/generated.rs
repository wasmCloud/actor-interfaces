extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
extern crate wapc_guest as guest;
use guest::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;

/// Used to register core message handlers
pub struct Handlers {}

impl Handlers {
    pub fn register_health_request(
        f: fn(HealthCheckRequest) -> HandlerResult<HealthCheckResponse>,
    ) {
        *HEALTH_REQUEST.write().unwrap() = Some(f);
        register_function(&"HealthRequest", health_request_wrapper);
    }
}

lazy_static! {
    static ref HEALTH_REQUEST: RwLock<Option<fn(HealthCheckRequest) -> HandlerResult<HealthCheckResponse>>> =
        RwLock::new(None);
}

fn health_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<HealthCheckRequest>(input_payload)?;
    let lock = HEALTH_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CapabilityConfiguration {
    #[serde(rename = "module")]
    pub module: String,
    #[serde(rename = "values")]
    pub values: std::collections::HashMap<String, String>,
}

/// A request sent to the actor by the host itself in order to determine
/// health status
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct HealthCheckRequest {
    #[serde(rename = "placeholder")]
    pub placeholder: bool,
}

/// All actors must return a health check response to the host upon
/// receipt of a health request. Returning in `Err` indicates total
/// actor failure, while returning a valid response with the `healthy`
/// flag set to false indicates that the actor has somehow detected that
/// it cannot perform its given task
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct HealthCheckResponse {
    #[serde(rename = "healthy")]
    pub healthy: bool,
    #[serde(rename = "message")]
    pub message: String,
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