extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

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
    /// Create a container in a blobstore. Returns the container created if successful
    pub fn create_container(&self, id: String) -> HandlerResult<Container> {
        let input_args = CreateContainerArgs { id };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "CreateContainer",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<Container>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Remove a container from a blobstore
    pub fn remove_container(&self, id: String) -> HandlerResult<BlobstoreResult> {
        let input_args = RemoveContainerArgs { id };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "RemoveContainer",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobstoreResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Remove an object from a blobstore
    pub fn remove_object(
        &self,
        id: String,
        container_id: String,
    ) -> HandlerResult<BlobstoreResult> {
        let input_args = RemoveObjectArgs { id, container_id };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "RemoveObject",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobstoreResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Returns a list of blobs that are present in the specified container
    pub fn list_objects(&self, container_id: String) -> HandlerResult<BlobList> {
        let input_args = ListObjectsArgs { container_id };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "ListObjects",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobList>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Upload a file chunk to a blobstore, which may only be part of a full file. This
    /// must be called AFTER the StartUpload operation. Chunks should be small, as
    /// memory over a few megabytes may exceed the wasm memory allocation.
    pub fn upload_chunk(&self, chunk: FileChunk) -> HandlerResult<BlobstoreResult> {
        let input_args = UploadChunkArgs { chunk };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "UploadChunk",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobstoreResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Issue a request to start a download from a blobstore. Chunks will be sent to the
    /// function that's registered with the ReceiveChunk operation.
    pub fn start_download(
        &self,
        blob_id: String,
        container_id: String,
        chunk_size: u64,
        context: String,
    ) -> HandlerResult<BlobstoreResult> {
        let input_args = StartDownloadArgs {
            blob_id,
            container_id,
            chunk_size,
            context,
        };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "StartDownload",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobstoreResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Begin the upload process with the first chunk of a full file. Subsequent chunks
    /// should be uploaded with the UploadChunk operation.
    pub fn start_upload(&self, chunk: FileChunk) -> HandlerResult<BlobstoreResult> {
        let input_args = StartUploadArgs { chunk };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "StartUpload",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobstoreResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    /// Retreives information about a blob
    pub fn get_object_info(&self, blob_id: String, container_id: String) -> HandlerResult<Blob> {
        let input_args = GetObjectInfoArgs {
            blob_id,
            container_id,
        };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "GetObjectInfo",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<Blob>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    /// Defines a handler for incoming chunks forwarded by a wasmcloud:blobstore
    /// provider. Chunks may not be received in order.
    pub fn register_receive_chunk(f: fn(FileChunk) -> HandlerResult<()>) {
        *RECEIVE_CHUNK.write().unwrap() = Some(f);
        register_function(&"ReceiveChunk", receive_chunk_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static::lazy_static! {
static ref RECEIVE_CHUNK: std::sync::RwLock<Option<fn(FileChunk) -> HandlerResult<()>>> = std::sync::RwLock::new(None);
}

#[cfg(feature = "guest")]
fn receive_chunk_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<ReceiveChunkArgs>(input_payload)?;
    let lock = RECEIVE_CHUNK.read().unwrap().unwrap();
    let result = lock(input.chunk)?;
    serialize(result)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CreateContainerArgs {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RemoveContainerArgs {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RemoveObjectArgs {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "container_id")]
    pub container_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ListObjectsArgs {
    #[serde(rename = "container_id")]
    pub container_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct UploadChunkArgs {
    #[serde(rename = "chunk")]
    pub chunk: FileChunk,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StartDownloadArgs {
    #[serde(rename = "blob_id")]
    pub blob_id: String,
    #[serde(rename = "container_id")]
    pub container_id: String,
    #[serde(rename = "chunk_size")]
    pub chunk_size: u64,
    #[serde(rename = "context")]
    pub context: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StartUploadArgs {
    #[serde(rename = "chunk")]
    pub chunk: FileChunk,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GetObjectInfoArgs {
    #[serde(rename = "blob_id")]
    pub blob_id: String,
    #[serde(rename = "container_id")]
    pub container_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ReceiveChunkArgs {
    #[serde(rename = "chunk")]
    pub chunk: FileChunk,
}

/// Represents a single chunk that may comprise part of a file or an entire file.
/// The fields for sequence number, total bytes and chunk bytes should be used to
/// determine the chunk order, as well as the optional context field.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct FileChunk {
    #[serde(rename = "sequenceNo")]
    pub sequence_no: u64,
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "totalBytes")]
    pub total_bytes: u64,
    #[serde(rename = "chunkSize")]
    pub chunk_size: u64,
    #[serde(rename = "context")]
    pub context: Option<String>,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "chunkBytes")]
    pub chunk_bytes: Vec<u8>,
}

/// A container is a logical grouping of blobs, similar to a directory in a file
/// system.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Container {
    #[serde(rename = "id")]
    pub id: String,
}

/// A wrapper object around a list of containers.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ContainerList {
    #[serde(rename = "containers")]
    pub containers: Vec<Container>,
}

/// A blob is a representation of an object in a blobstore, similar to a file in a
/// file system.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Blob {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "byteSize")]
    pub byte_size: u64,
}

/// A wrapper object around a list of blobs.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct BlobList {
    #[serde(rename = "blobs")]
    pub blobs: Vec<Blob>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Transfer {
    #[serde(rename = "blobId")]
    pub blob_id: String,
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "chunkSize")]
    pub chunk_size: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    #[serde(rename = "totalChunks")]
    pub total_chunks: u64,
    #[serde(rename = "context")]
    pub context: Option<String>,
}

/// Used to return success and error information for common blobstore operations
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct BlobstoreResult {
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
