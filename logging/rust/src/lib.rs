#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # wasmCloud Logging Actor Interface
//!
//! This crate provides an abstraction over the `wasmcloud:logging` contract. This
//! allows actors to use normal log macros (like `info!`, `warn!`, `error!`, etc)
//! to write logs from within the actor.
//!
//! Example:
//! ```rust
//! extern crate wasmcloud_actor_http_server as http;
//! extern crate wasmcloud_actor_logging as logging;
//! extern crate wasmcloud_actor_core as actor;
//! use wapc_guest::HandlerResult;
//! use http::{Request, Response, Handlers};
//! use log::{info, warn, error, trace, debug};
//!
//! #[actor::init]
//! pub fn init() {
//!     http::Handlers::register_handle_request(method_logger);
//!     /// Initialize the logger to enable log macros
//!     logging::enable_macros();
//! }
//!
//! /// Actor must be signed with `wasmcloud:logging` to log messages
//! fn method_logger(msg: http::Request) -> HandlerResult<http::Response> {
//!     /// Logs can be directly written via `write_log`
//!     logging::default().write_log("", "trace", "Coercing Rust String to str");
//!     
//!     /// After initialization, logs can be directly written from the actor using macros
//!     match &*msg.method {
//!         "GET" => info!("Received a GET request"),
//!         "POST" => info!("Received a POST request"),
//!         "PUT" => info!("Received a PUT request"),
//!         "DELETE" => warn!("Received a DELETE request"),
//!         req => error!("Received an unsupported HTTP Request: {}", req),
//!     };
//!     debug!("Finished matching HTTP method, returning OK");
//!     Ok(http::Response::ok())
//! }
//! ```

mod generated;
#[allow(unused_imports)]
pub use generated::*;

// The operation used to request writing a log
pub const OP_LOG: &str = "WriteLog";

#[cfg(feature = "guest")]
#[doc(hidden)]
static LOG_LEVELS: [&str; 5] = ["error", "warn", "info", "debug", "trace"];

#[cfg(feature = "guest")]
impl Host {
    /// Writes a log message to specified target and level
    ///
    /// # Arguments
    ///
    /// * `target` - Used to filter logs to a specific target, e.g. actor name. Can be left blank
    /// * `level` - Log level, accepts `error`, `warn`, `info`, `debug`, `trace`. Defaults to `info`
    /// * `text` - Text to log
    ///
    pub fn write_log(&self, target: &str, level: &str, text: &str) -> HandlerResult<()> {
        let log_level = if LOG_LEVELS.contains(&level.to_ascii_lowercase().as_str()) {
            level
        } else {
            "info"
        };
        self._write_log(target.to_string(), log_level.to_string(), text.to_string())
    }
}

// Begin implementation of automatic log macro interception

#[cfg(feature = "guest")]
use lazy_static::lazy_static;
#[cfg(feature = "guest")]
use log::{Metadata, Record};
#[cfg(feature = "guest")]
use std::sync::{Arc, RwLock};
#[cfg(feature = "guest")]
use wapc_guest::HandlerResult;

#[cfg(feature = "guest")]
lazy_static! {
    static ref CURRENT_BINDING: Arc<RwLock<String>> = Arc::new(RwLock::new("default".to_string()));
}

#[cfg(feature = "guest")]
static LOGGER: Host = Host {};

/// Initializes the logger to use standard log macros
///
/// This function must be called before attempting to use log macros
/// such as `info!` or `debug!` or the logs will not be written by the logger
#[cfg(feature = "guest")]
pub fn enable_macros() {
    if log::set_logger(&LOGGER).is_ok() {};
    log::set_max_level(log::LevelFilter::Trace);
}

#[cfg(feature = "guest")]
fn set_binding(binding: &str) {
    *CURRENT_BINDING.write().unwrap() = binding.to_string();
}

#[cfg(feature = "guest")]
impl log::Log for Host {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        use log::Level::*;
        let level = match record.level() {
            Error => "error",
            Warn => "warn",
            Info => "info",
            Debug => "debug",
            Trace => "trace",
        };
        let _ = self._write_log(
            record.target().to_string(),
            level.to_string(),
            format!("{}", record.args()),
        );
    }

    fn flush(&self) {}
}
