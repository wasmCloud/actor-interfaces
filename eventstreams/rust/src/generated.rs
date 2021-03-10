extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
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

#[cfg(feature = "guest")]
/// Creates a named host binding for the event streams capability
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

#[cfg(feature = "guest")]
/// Creates the default host binding for the event streams capability
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    /// Writes a map of key-value pairs to the given event stream
    pub fn write_event(
        &self,
        stream_id: String,
        values: std::collections::HashMap<String, String>,
    ) -> HandlerResult<EventAck> {
        let input_args = WriteEventArgs {
            stream_id: stream_id,
            values: values,
        };
        host_call(
            &self.binding,
            "wasmcloud:eventstreams",
            "WriteEvent",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<EventAck>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Queries the stream per the parameters in the supplied query
    pub fn query_stream(&self, query: StreamQuery) -> HandlerResult<EventList> {
        host_call(
            &self.binding,
            "wasmcloud:eventstreams",
            "QueryStream",
            &serialize(query)?,
        )
        .map(|vec| {
            let resp = deserialize::<EventList>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct WriteEventArgs {
    #[serde(rename = "streamId")]
    pub stream_id: String,
    #[serde(rename = "values")]
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Event {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "streamId")]
    pub stream_id: String,
    #[serde(rename = "values")]
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct EventAck {
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
    #[serde(rename = "error")]
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct EventList {
    #[serde(rename = "events")]
    pub events: Vec<Event>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StreamQuery {
    #[serde(rename = "streamId")]
    pub stream_id: String,
    #[serde(rename = "range")]
    pub range: Option<TimeRange>,
    #[serde(rename = "count")]
    pub count: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct TimeRange {
    #[serde(rename = "minTime")]
    pub min_time: u64,
    #[serde(rename = "maxTime")]
    pub max_time: u64,
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
