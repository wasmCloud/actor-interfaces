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
    pub fn publish(
        &self,
        subject: String,
        reply_to: String,
        body: Vec<u8>,
    ) -> HandlerResult<PublishResponse> {
        let input_args = PublishArgs {
            subject,
            reply_to,
            body,
        };
        host_call(
            &self.binding,
            "wasmcloud:messaging",
            "Publish",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<PublishResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn request(
        &self,
        subject: String,
        body: Vec<u8>,
        timeout: i64,
    ) -> HandlerResult<BrokerMessage> {
        let input_args = RequestArgs {
            subject,
            body,
            timeout,
        };
        host_call(
            &self.binding,
            "wasmcloud:messaging",
            "Request",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BrokerMessage>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn deliver_message(&self, message: BrokerMessage) -> HandlerResult<()> {
        host_call(
            &self.binding,
            "wasmcloud:messaging",
            "DeliverMessage",
            &serialize(message)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    pub fn register_publish(f: fn(String, String, Vec<u8>) -> HandlerResult<PublishResponse>) {
        *PUBLISH.write().unwrap() = Some(f);
        register_function(&"Publish", publish_wrapper);
    }
    pub fn register_request(f: fn(String, Vec<u8>, i64) -> HandlerResult<BrokerMessage>) {
        *REQUEST.write().unwrap() = Some(f);
        register_function(&"Request", request_wrapper);
    }
    pub fn register_deliver_message(f: fn(BrokerMessage) -> HandlerResult<()>) {
        *DELIVER_MESSAGE.write().unwrap() = Some(f);
        register_function(&"DeliverMessage", deliver_message_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref PUBLISH: RwLock<Option<fn(String, String, Vec<u8>) -> HandlerResult<PublishResponse>>> =
        RwLock::new(None);
    static ref REQUEST: RwLock<Option<fn(String, Vec<u8>, i64) -> HandlerResult<BrokerMessage>>> =
        RwLock::new(None);
    static ref DELIVER_MESSAGE: RwLock<Option<fn(BrokerMessage) -> HandlerResult<()>>> =
        RwLock::new(None);
}

#[cfg(feature = "guest")]
fn publish_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<PublishArgs>(input_payload)?;
    let lock = PUBLISH.read().unwrap().unwrap();
    let result = lock(input.subject, input.reply_to, input.body)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<RequestArgs>(input_payload)?;
    let lock = REQUEST.read().unwrap().unwrap();
    let result = lock(input.subject, input.body, input.timeout)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn deliver_message_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<BrokerMessage>(input_payload)?;
    let lock = DELIVER_MESSAGE.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PublishArgs {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "body")]
    pub body: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RequestArgs {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "body")]
    pub body: Vec<u8>,
    #[serde(rename = "timeout")]
    pub timeout: i64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PublishResponse {
    #[serde(rename = "published")]
    pub published: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct BrokerMessage {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
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
