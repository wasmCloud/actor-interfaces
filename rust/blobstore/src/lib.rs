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

// extern crate wapc_guest as guest;
// use guest::prelude::*;

// #[no_mangle]
// pub fn wapc_init() {
//     Handlers::register_create_container(create_container);
//     Handlers::register_remove_container(remove_container);
//     Handlers::register_remove_object(remove_object);
//     Handlers::register_list_objects(list_objects);
//     Handlers::register_upload_chunk(upload_chunk);
//     Handlers::register_start_download(start_download);
//     Handlers::register_start_upload(start_upload);
//     Handlers::register_receive_chunk(receive_chunk);
//     Handlers::register_get_object_info(get_object_info);
// }

// fn create_container(_container: Container) -> HandlerResult<Container> {
//     Ok(Container::default()) // TODO: Provide implementation.
// }

// fn remove_container(_container: Container) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

// fn remove_object(_blob: Blob) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

// fn list_objects(_container: Container) -> HandlerResult<BlobList> {
//     Ok(BlobList::default()) // TODO: Provide implementation.
// }

// fn upload_chunk(_chunk: FileChunk) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

// fn start_download(_request: StreamRequest) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

// fn start_upload(_blob: FileChunk) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

// fn receive_chunk(_chunk: FileChunk) -> HandlerResult<()> {
//     Ok(()) // TODO: Provide implementation.
// }

// fn get_object_info(_blob: Blob) -> HandlerResult<Blob> {
//     Ok(Blob::default()) // TODO: Provide implementation.
// }

pub mod generated;
#[allow(unused_imports)]
use generated::*;

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
