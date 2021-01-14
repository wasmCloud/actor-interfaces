extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

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

/// Creates a named host binding for the key-value store capability
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding for the key-value store capability
pub fn default() -> Host {
    Host::default()
}

impl Host {
    /*pub fn session_started(&self, session: String) -> HandlerResult<bool> {
        let input_args = SessionStartedArgs { session: session };
        host_call(
            &self.binding,
            "wasmcloud:telnet",
            "SessionStarted",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<bool>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn receive_text(&self, session: String, text: String) -> HandlerResult<bool> {
        let input_args = ReceiveTextArgs {
            session: session,
            text: text,
        };
        host_call(
            &self.binding,
            "wasmcloud:telnet",
            "ReceiveText",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<bool>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    } */

    /// Sends a string of text to a given session. The provider is not responsible for
    /// indicating if this is a valid session or not. The telnet provider will not automatically
    /// add newlines or carriage returns.
    pub fn send_text(&self, session: String, text: String) -> HandlerResult<bool> {
        let input_args = SendTextArgs {
            session: session,
            text: text,
        };
        host_call(
            &self.binding,
            "wasmcloud:telnet",
            "SendText",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<bool>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

pub struct Handlers {}

impl Handlers {
    pub fn register_session_started(f: fn(String) -> HandlerResult<bool>) {
        *SESSION_STARTED.write().unwrap() = Some(f);
        register_function(&"SessionStarted", session_started_wrapper);
    }
    pub fn register_receive_text(f: fn(String, String) -> HandlerResult<bool>) {
        *RECEIVE_TEXT.write().unwrap() = Some(f);
        register_function(&"ReceiveText", receive_text_wrapper);
    }
    // This is not a valid handler , the provider doesn't support it.
    //pub fn register_send_text(f: fn(String, String) -> HandlerResult<bool>) {
    //*SEND_TEXT.write().unwrap() = Some(f);
    //register_function(&"SendText", send_text_wrapper);
    //}
}

lazy_static! {
    static ref SESSION_STARTED: RwLock<Option<fn(String) -> HandlerResult<bool>>> =
        RwLock::new(None);
    static ref RECEIVE_TEXT: RwLock<Option<fn(String, String) -> HandlerResult<bool>>> =
        RwLock::new(None);
    static ref SEND_TEXT: RwLock<Option<fn(String, String) -> HandlerResult<bool>>> =
        RwLock::new(None);
}

fn session_started_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<SessionStartedArgs>(input_payload)?;
    let lock = SESSION_STARTED.read().unwrap().unwrap();
    let result = lock(input.session)?;
    Ok(serialize(result)?)
}

fn receive_text_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<ReceiveTextArgs>(input_payload)?;
    let lock = RECEIVE_TEXT.read().unwrap().unwrap();
    let result = lock(input.session, input.text)?;
    Ok(serialize(result)?)
}

fn send_text_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<SendTextArgs>(input_payload)?;
    let lock = SEND_TEXT.read().unwrap().unwrap();
    let result = lock(input.session, input.text)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SessionStartedArgs {
    #[serde(rename = "session")]
    pub session: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ReceiveTextArgs {
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SendTextArgs {
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "text")]
    pub text: String,
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
