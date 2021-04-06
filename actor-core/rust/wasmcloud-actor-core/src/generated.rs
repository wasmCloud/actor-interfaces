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
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    /// A function that can respond to health checks
    pub fn register_health_request(
        f: fn(HealthCheckRequest) -> HandlerResult<HealthCheckResponse>,
    ) {
        *HEALTH_REQUEST.write().unwrap() = Some(f);
        register_function(&"HealthRequest", health_request_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref HEALTH_REQUEST: RwLock<Option<fn(HealthCheckRequest) -> HandlerResult<HealthCheckResponse>>> =
        RwLock::new(None);
}

#[cfg(feature = "guest")]
fn health_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<HealthCheckRequest>(input_payload)?;
    let lock = HEALTH_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

/// Represents the data sent to a capability provider at link time
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CapabilityConfiguration {
    /// The module name
    #[serde(rename = "module")]
    pub module: String,
    /// A map of values that represent the configuration properties
    #[serde(rename = "values")]
    pub values: std::collections::HashMap<String, String>,
}

/// A request sent to the actor by the host itself in order to determine health
/// status
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct HealthCheckRequest {
    /// TODO: Figure out what goes here
    #[serde(rename = "placeholder")]
    pub placeholder: bool,
}

/// All actors must return a health check response to the host upon receipt of a
/// health request. Returning in `Err` indicates total actor failure, while
/// returning a valid response with the `healthy` flag set to false indicates that
/// the actor has somehow detected that it cannot perform its given task
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct HealthCheckResponse {
    /// A flag that indicates the the actor is healthy
    #[serde(rename = "healthy")]
    pub healthy: bool,
    /// A message containing additional information about the actors health
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
