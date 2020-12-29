mod generated;
use generated::Host;
pub use generated::{Handlers,PublishArgs,RequestArgs,PublishResponse,BrokerMessage};
use wapc_guest::HandlerResult;
pub struct MessageBrokerHostBinding {
  binding: String,
}
/// Create a new named message broker host binding
pub fn host(binding: &str) -> MessageBrokerHostBinding {
  MessageBrokerHostBinding {
      binding: binding.to_string(),
  }
}

/// Create a default message broker host binding
pub fn default() -> MessageBrokerHostBinding {
  MessageBrokerHostBinding {
      binding: "default".to_string(),
  }
}

impl MessageBrokerHostBinding {
  /// Publishes a message on a given subject with an optional reply subject
  pub fn publish(
      &self,
      subject: &str,
      reply_to: Option<&str>,
      payload: &[u8],
  ) -> HandlerResult<PublishResponse> {
      Host::default().publish(subject.to_string(),
      reply_to.map_or("".to_string(), |r| r.to_string()),
      payload.to_vec())
  }

  /// Publishes a message and expects a reply to come back within a given timeout (in milliseconds)
  pub fn request(
      &self,
      subject: &str,
      payload: &[u8],
      timeout_ms: u64,
  ) -> HandlerResult<BrokerMessage> {
      Host::default().request(subject.to_string(),
      payload.to_vec(),timeout_ms as _)
  }
}
