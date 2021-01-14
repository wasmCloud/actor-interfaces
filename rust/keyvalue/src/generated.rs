extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

/// The abstraction of the key-value host capability
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

/// Creates a named host binding for the key-value store capability
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding for the key-value store capability
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    /// Retrieves a value stored in a given key
    pub fn get(&self, key: String) -> HandlerResult<GetResponse> {
        let input_args = GetArgs { key: key };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Get",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<GetResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Adds a number to the value stored at a given key. Will return an error
    /// if you attempt to add a value to an existing key that is holding a string
    pub fn add(&self, key: String, value: i32) -> HandlerResult<AddResponse> {
        let input_args = AddArgs {
            key: key,
            value: value,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Add",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<AddResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Sets the given value for the key with an optional expiration period. Set expiration to 0
    /// for a value that does not expire.
    pub fn set(&self, key: String, value: String, expires: i32) -> HandlerResult<SetResponse> {
        let input_args = SetArgs {
            key: key,
            value: value,
            expires: expires,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Set",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Deletes the specified key
    pub fn del(&self, key: String) -> HandlerResult<DelResponse> {
        let input_args = DelArgs { key: key };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Del",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<DelResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Clears the list at the given key
    pub fn clear(&self, key: String) -> HandlerResult<DelResponse> {
        let input_args = ClearArgs { key: key };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Clear",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<DelResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Retrieves a range of values stored in a list key
    pub fn range(&self, key: String, start: i32, stop: i32) -> HandlerResult<ListRangeResponse> {
        let input_args = RangeArgs {
            key: key,
            start: start,
            stop: stop,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Range",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<ListRangeResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Pushes a value onto a list
    pub fn push(&self, key: String, value: String) -> HandlerResult<ListResponse> {
        let input_args = PushArgs {
            key: key,
            value: value,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "Push",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<ListResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Deletes the given item from a specified list
    pub fn list_item_delete(&self, key: String, value: String) -> HandlerResult<ListResponse> {
        let input_args = ListItemDeleteArgs {
            key: key,
            value: value,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "ListItemDelete",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<ListResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Adds a value to a set
    pub fn set_add(&self, key: String, value: String) -> HandlerResult<SetOperationResponse> {
        let input_args = SetAddArgs {
            key: key,
            value: value,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "SetAdd",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetOperationResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Removes an item from a set
    pub fn set_remove(&self, key: String, value: String) -> HandlerResult<SetOperationResponse> {
        let input_args = SetRemoveArgs {
            key: key,
            value: value,
        };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "SetRemove",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetOperationResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Returns the union of all sets specified by the list of keys
    pub fn set_union(&self, keys: Vec<String>) -> HandlerResult<SetQueryResponse> {
        let input_args = SetUnionArgs { keys: keys };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "SetUnion",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetQueryResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Returns the intersection of all sets specified by the list of keys
    pub fn set_intersection(&self, keys: Vec<String>) -> HandlerResult<SetQueryResponse> {
        let input_args = SetIntersectionArgs { keys: keys };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "SetIntersection",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetQueryResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Returns the list of members in a set
    pub fn set_query(&self, key: String) -> HandlerResult<SetQueryResponse> {
        let input_args = SetQueryArgs { key: key };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "SetQuery",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<SetQueryResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Indicates if a given key exists. The "value" field will be empty in this response
    pub fn key_exists(&self, key: String) -> HandlerResult<GetResponse> {
        let input_args = KeyExistsArgs { key: key };
        host_call(
            &self.binding,
            "wasmcloud:keyvalue",
            "KeyExists",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<GetResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GetArgs {
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct AddArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "expires")]
    pub expires: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DelArgs {
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ClearArgs {
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RangeArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "start")]
    pub start: i32,
    #[serde(rename = "stop")]
    pub stop: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PushArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ListItemDeleteArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetAddArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetRemoveArgs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetUnionArgs {
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetIntersectionArgs {
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetQueryArgs {
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct KeyExistsArgs {
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GetResponse {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "exists")]
    pub exists: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct AddResponse {
    #[serde(rename = "value")]
    pub value: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct DelResponse {
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ListRangeResponse {
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ListResponse {
    #[serde(rename = "newCount")]
    pub new_count: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetResponse {
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetOperationResponse {
    #[serde(rename = "new_count")]
    pub new_count: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SetQueryResponse {
    #[serde(rename = "values")]
    pub values: Vec<String>,
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
