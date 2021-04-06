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
    /// Sends a string of text to a given session. The provider is not responsible for
    /// indicating if this is a valid session or not. The telnet provider will not
    /// automatically add newlines or carriage returns.
    pub fn send_text(&self, session: String, text: String) -> HandlerResult<TelnetResult> {
        let input_args = SendTextArgs { session, text };
        host_call(
            &self.binding,
            "wasmcloud:telnet",
            "SendText",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<TelnetResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    /// Indicates that a telnet client has connected with a given session ID
    pub fn register_session_started(f: fn(String) -> HandlerResult<TelnetResult>) {
        *SESSION_STARTED.write().unwrap() = Some(f);
        register_function(&"SessionStarted", session_started_wrapper);
    }
    /// Invoked when a given session sends a string of text through their telnet session
    pub fn register_receive_text(f: fn(String, String) -> HandlerResult<TelnetResult>) {
        *RECEIVE_TEXT.write().unwrap() = Some(f);
        register_function(&"ReceiveText", receive_text_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref SESSION_STARTED: RwLock<Option<fn(String) -> HandlerResult<TelnetResult>>> =
        RwLock::new(None);
    static ref RECEIVE_TEXT: RwLock<Option<fn(String, String) -> HandlerResult<TelnetResult>>> =
        RwLock::new(None);
}

#[cfg(feature = "guest")]
fn session_started_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<SessionStartedArgs>(input_payload)?;
    let lock = SESSION_STARTED.read().unwrap().unwrap();
    let result = lock(input.session)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn receive_text_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<ReceiveTextArgs>(input_payload)?;
    let lock = RECEIVE_TEXT.read().unwrap().unwrap();
    let result = lock(input.session, input.text)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SendTextArgs {
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "text")]
    pub text: String,
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

/// Result type for telnet operations including a success value and optional error
/// information
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct TelnetResult {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "error")]
    pub error: Option<String>,
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
