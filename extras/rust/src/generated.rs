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

/// An abstraction around the host's extras capability
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

/// Creates a named host binding for the extras capability
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding for the extras capability
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    /// Requests a UUID/GUID from the host. If, for any reason, the host is unable to produce a UUID, this will return `None`
    pub fn request_guid(&self) -> HandlerResult<Option<String>> {
        self.request_guid_raw(GeneratorRequest {
            guid: true,
            sequence: false,
            random: false,
            min: 0,
            max: 0,
        })
    }

    fn request_guid_raw(&self, req: GeneratorRequest) -> HandlerResult<Option<String>> {
        host_call(
            &self.binding,
            "wasmcloud:extras",
            "RequestGuid",
            &serialize(req)?,
        )
        .map(|vec| {
            let resp = deserialize::<GeneratorResult>(vec.as_ref()).unwrap();
            resp.guid
        })
        .map_err(|e| e.into())
    }

    /// Requests a random number from the host within the given range
    pub fn request_random(&self, min: u32, max: u32) -> HandlerResult<u32> {
        self.request_random_raw(GeneratorRequest {
            guid: false,
            sequence: false,
            random: true,
            min,
            max,
        })
    }

    fn request_random_raw(&self, req: GeneratorRequest) -> HandlerResult<u32> {
        host_call(
            &self.binding,
            "wasmcloud:extras",
            "RequestRandom",
            &serialize(req)?,
        )
        .map(|vec| {
            let resp = deserialize::<GeneratorResult>(vec.as_ref()).unwrap();
            resp.random_number
        })
        .map_err(|e| e.into())
    }

    /// Requests a sequence number from the host. Sequence numbers are monotonically increasing within the lifetime of a single host process.
    /// They are not guaranteed to be globally unique and they can potentially reset from 0 at runtime after a crash.
    pub fn request_sequence(&self) -> HandlerResult<u64> {
        self.request_sequence_raw(GeneratorRequest {
            guid: false,
            sequence: true,
            random: false,
            min: 0,
            max: 0,
        })
    }

    fn request_sequence_raw(&self, req: GeneratorRequest) -> HandlerResult<u64> {
        host_call(
            &self.binding,
            "wasmcloud:extras",
            "RequestSequence",
            &serialize(req)?,
        )
        .map(|vec| {
            let resp = deserialize::<GeneratorResult>(vec.as_ref()).unwrap();
            resp.sequence_number
        })
        .map_err(|e| e.into())
    }
}

#[doc(hidden)]
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GeneratorResult {
    #[serde(rename = "guid")]
    pub guid: Option<String>,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u64,
    #[serde(rename = "random_number")]
    pub random_number: u32,
}

#[doc(hidden)]
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GeneratorRequest {
    #[serde(rename = "guid")]
    pub guid: bool,
    #[serde(rename = "sequence")]
    pub sequence: bool,
    #[serde(rename = "random")]
    pub random: bool,
    #[serde(rename = "min")]
    pub min: u32,
    #[serde(rename = "max")]
    pub max: u32,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
#[doc(hidden)]
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
#[doc(hidden)]
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
