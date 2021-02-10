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
    /// Publishes a message on a given subject with an optional reply subject
    ///
    /// # Arguments
    ///
    /// * `subject` - Message subject
    /// * `reply_to` - Subject to receive message replies. Can be left blank for no reply subject
    /// * `body` - Message payload
    ///
    /// # Example
    /// ```rust
    /// extern crate wasmcloud_actor_messaging as messaging;
    /// fn send_message() {
    ///     let subject = "first.app".to_string();
    ///     let reply_to = "".to_string();
    ///     let body = "hello world".to_string().into_bytes();
    ///     
    ///     messaging::default().publish(subject, reply_to, body);
    /// }
    /// ```
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

    /// Publishes a message and expects a reply within a given timeout
    ///
    /// # Arguments
    ///
    /// * `subject` - Message subject
    /// * `body` - Message payload
    /// * `timeout` - Timeout, in milliseconds, to wait for a message reply
    ///
    /// # Example
    /// ```rust
    /// extern crate wasmcloud_actor_messaging as messaging;
    /// fn hello_there() {
    ///     let subject = "first.app".to_string();
    ///     let body = "hello?".to_string().into_bytes();
    ///     let timeout = 1000;
    ///     
    ///     let response = messaging::default().request(subject, body, timeout).unwrap();
    ///     assert_eq!(String::from_utf8(response.body).unwrap(), "world!");
    /// }
    /// ```
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
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    /// Registers a function to be invoked upon receiving a message.
    /// This function should be called with the `wapc_init` block of an
    /// actor module.
    ///
    /// # Example
    /// ```rust
    /// extern crate wapc_guest as guest;
    /// extern crate wasmcloud_actor_messaging as messaging;
    /// use guest::prelude::*;
    ///
    /// #[no_mangle]
    /// pub fn wapc_init() {
    ///     messaging::Handlers::register_handle_message(handle_message);
    /// }
    ///
    /// fn handle_message(_msg: messaging::BrokerMessage) -> HandlerResult<()> {
    ///     todo!()
    /// }
    /// ```
    pub fn register_handle_message(f: fn(BrokerMessage) -> HandlerResult<()>) {
        *HANDLE_MESSAGE.write().unwrap() = Some(f);
        register_function(&"HandleMessage", handle_message_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref HANDLE_MESSAGE: RwLock<Option<fn(BrokerMessage) -> HandlerResult<()>>> =
        RwLock::new(None);
}

#[cfg(feature = "guest")]
fn handle_message_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<BrokerMessage>(input_payload)?;
    let lock = HANDLE_MESSAGE.read().unwrap().unwrap();
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
