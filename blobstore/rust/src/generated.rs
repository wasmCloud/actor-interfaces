extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
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

#[cfg(feature = "guest")]
/// Creates a reference a blob store capability provider with the given link name
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

#[cfg(feature = "guest")]
/// Creates a reference to the default blob store capability provider
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    /// Creates a new container with the given ID
    pub fn create_container(&self, id: String) -> HandlerResult<Container> {
        let input_args = CreateContainerArgs { id: id };
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

    /// Removes the indicated container, if possible
    pub fn remove_container(&self, id: String) -> HandlerResult<BlobstoreResult> {
        let input_args = RemoveContainerArgs { id: id };
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

    /// Removes a blob from within the given container, if possible
    pub fn remove_object(
        &self,
        id: String,
        container_id: String,
    ) -> HandlerResult<BlobstoreResult> {
        let input_args = RemoveObjectArgs {
            id: id,
            container_id: container_id,
        };
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

    /// Lists all of the blob entries within the given container
    pub fn list_objects(&self, container_id: String) -> HandlerResult<BlobList> {
        let input_args = ListObjectsArgs {
            container_id: container_id,
        };
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

    /// Uploads a chunk of a file to the host. Must only be called _after_ calling `start_upload`
    pub fn upload_chunk(&self, chunk: FileChunk) -> HandlerResult<()> {
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "UploadChunk",
            &serialize(chunk)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    /// Indicates to the capability provider that the actor would like to start receiving chunks for the
    /// given blob in the indicated container
    pub fn start_download(
        &self,
        blob_id: String,
        container_id: String,
        chunk_size: u64,
        context: Option<String>,
    ) -> HandlerResult<BlobstoreResult> {
        let input_args = StartDownloadArgs {
            blob_id: blob_id,
            container_id: container_id,
            chunk_size: chunk_size,
            context: context,
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

    /// Starts an upload with the capability provider. This must be called before invoking `upload_chunk`
    pub fn start_upload(&self, blob: FileChunk) -> HandlerResult<BlobstoreResult> {
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "StartUpload",
            &serialize(blob)?,
        )
        .map(|vec| {
            let resp = deserialize::<BlobstoreResult>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    /// Retrieves metadata associated with the given blob in the indicated container
    pub fn get_object_info(&self, blob_id: String, container_id: String) -> HandlerResult<Blob> {
        let input_args = GetObjectInfoArgs {
            blob_id: blob_id,
            container_id: container_id,
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
    /// Sets a handler for receiving file chunks from the capability provider
    pub fn register_receive_chunk(f: fn(FileChunk) -> HandlerResult<()>) {
        *RECEIVE_CHUNK.write().unwrap() = Some(f);
        register_function(&"ReceiveChunk", receive_chunk_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref RECEIVE_CHUNK: RwLock<Option<fn(FileChunk) -> HandlerResult<()>>> =
        RwLock::new(None);
}

#[cfg(feature = "guest")]
fn receive_chunk_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<FileChunk>(input_payload)?;
    let lock = RECEIVE_CHUNK.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
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
pub struct GetObjectInfoArgs {
    #[serde(rename = "blob_id")]
    pub blob_id: String,
    #[serde(rename = "container_id")]
    pub container_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct BlobstoreResult {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "error")]
    pub error: Option<String>,
}

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

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Container {
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ContainerList {
    #[serde(rename = "containers")]
    pub containers: Vec<Container>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Blob {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "byteSize")]
    pub byte_size: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct BlobList {
    #[serde(rename = "blobs")]
    pub blobs: Vec<Blob>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StreamRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "container")]
    pub container: Container,
    #[serde(rename = "chunkSize")]
    pub chunk_size: u64,
    #[serde(rename = "context")]
    pub context: Option<String>,
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
