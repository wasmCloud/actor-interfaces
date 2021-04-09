extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

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
    /// Request a Globally Unique Identifier
    pub fn request_guid(&self) -> HandlerResult<String> {
        host_call(&self.binding, "wasmcloud:extras", "RequestGuid", &vec![])
            .map(|vec| {
                let resp = deserialize::<String>(vec.as_ref()).unwrap();
                resp
            })
            .map_err(|e| e.into())
    }
    /// Request a random number with minimum and maximum parameters. Inclusivity depends
    /// on implementation and is not guaranteed on either end
    pub fn request_random(&self, min: u32, max: u32) -> HandlerResult<u32> {
        let input_args = RequestRandomArgs { min, max };
        host_call(
            &self.binding,
            "wasmcloud:extras",
            "RequestRandom",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<u32>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Request the next number in a monotonically increasing sequence, starting at 0
    pub fn request_sequence(&self) -> HandlerResult<u64> {
        host_call(
            &self.binding,
            "wasmcloud:extras",
            "RequestSequence",
            &vec![],
        )
        .map(|vec| {
            let resp = deserialize::<u64>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RequestRandomArgs {
    #[serde(rename = "min")]
    pub min: u32,
    #[serde(rename = "max")]
    pub max: u32,
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
