#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
//! # wasmCloud Blob Store Actor Interface
//!
//! This crate provides wasmCloud actors with an interface to the blobstore capability provider.
//! Actors using this interface must have the claim `wasmcloud:blobstore` in order to have
//! permission to communicate with the store.
//!
//! This generic protocol can be used to support capability providers like local blob storage,
//! Amazon S3, Azure blob storage, Google blob storage, and more.
//!
//! # Example:
//!
//! ```rust
//! extern crate wapc_guest as guest;
//! use guest::prelude::*;
//! use wasmcloud_actor_blobstore as blobstore;
//! use wasmcloud_actor_http_server as http;
//! use wasmcloud_actor_core::serialize;
//! use wasmcloud_actor_core as actor;
//! use actor::init;
//! use blobstore::*;
//! use serde_json::json;
//! use log::{error, info};
//!
//! #[actor::init]
//! fn init() {
//!     http::Handlers::register_handle_request(download_poem);
//!     blobstore::Handlers::register_receive_chunk(handle_chunk);
//! }
//!
//! /// Start the download of a blob (poem). Chunks will be streamed after download begins
//! fn download_poem(req: http::Request) -> HandlerResult<http::Response> {
//!     // replace `start_download` with `blobstore::default().start_download`
//!     match start_download(req.path, "poems".to_string(), 4096, None) {
//!         Ok(_) => Ok(http::Response::ok()),
//!         Err(_) => Err("Failed to initiate download of chunk".into())
//!     }
//! }
//!
//! /// Handle the incoming chunk as a poem "verse" and log the result
//! /// Note that these chunks can be received out of order, so the poem
//! /// in this case might be displayed in a different order and could look
//! /// funny if the verse continues across a chunk boundary
//! fn handle_chunk(chunk: FileChunk) -> HandlerResult<()> {
//!     let verse = String::from_utf8(chunk.chunk_bytes)?;
//!     info!("Poem {} part {}:\n{}", chunk.id, chunk.sequence_no, verse);
//!     Ok(())
//! }
//!
//!
//! # fn start_download(id: String, container_id: String, chunk_size: u64, context: Option<String>) -> HandlerResult<()> {
//! #   Ok(())
//! # }
//!
//! ```

mod generated;
#[allow(unused_imports)]
pub use generated::*;

/// Guest sends a Container to the capability provider, receives a Container back
pub const OP_CREATE_CONTAINER: &str = "CreateContainer";
/// Guest sends a Container to the capability provider, lack of error indicates success
pub const OP_REMOVE_CONTAINER: &str = "RemoveContainer";
/// Guest sends a Blob to the capability provider, lack of error indicates success
pub const OP_REMOVE_OBJECT: &str = "RemoveObject";
/// Guest sends a Container to the capability provider, receives a BlobList back
pub const OP_LIST_OBJECTS: &str = "ListObjects";
/// Guest sends a FileChunk to capability provider for storing as part of a Blob, lack of error indicates success
pub const OP_UPLOAD_CHUNK: &str = "UploadChunk";
/// Guest sends a StreamRequest to the capability provider, immediate termination w/success. Guest will then
/// start receiving OP_RECEIVE_CHUNK operations from the provider as chunks are streamed to the guest
pub const OP_START_DOWNLOAD: &str = "StartDownload";
/// Guest sends a metadata-carrying FileChunk to initiate an upload, lack of error is success
pub const OP_START_UPLOAD: &str = "StartUpload";
/// Guest will receive a FileChunk for each piece of a file requested to download
pub const OP_RECEIVE_CHUNK: &str = "ReceiveChunk";
/// Query information on a single blob. Guest sends an incomplete blob struct and gets a complete one in return
pub const OP_GET_OBJECT_INFO: &str = "GetObjectInfo";

impl Container {
    pub fn new(id: impl Into<String>) -> Self {
        Container { id: id.into() }
    }
}
