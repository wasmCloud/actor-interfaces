mod generated;
pub use generated::*;
// #[cfg(feature = "guest")]
// extern crate wapc_guest as guest;
// #[cfg(feature = "guest")]
// use guest::prelude::*;

// #[no_mangle]
// pub fn wapc_init() {
//     Handlers::register_publish(publish);
//     Handlers::register_request(request);
//     Handlers::register_deliver_message(deliver_message);
// }

// fn publish(_subject: String, _replyTo: String, _body: Vec<u8>) -> HandlerResult<PublishResponse> {
//     Ok(PublishResponse::default()) // TODO: Provide implementation.
// }

// fn request(_subject: String, _body: Vec<u8>, _timeout: i64) -> HandlerResult<BrokerMessage> {
//     Ok(BrokerMessage::default()) // TODO: Provide implementation.
// }

// fn deliver_message(_message: BrokerMessage) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }
