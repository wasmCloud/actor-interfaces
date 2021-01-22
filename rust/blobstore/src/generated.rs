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
    pub fn create_container(&self, container: Container) -> HandlerResult<Container> {
        let input_args = CreateContainerArgs {
            container_id: container,
        };
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

    pub fn remove_container(&self, container: Container) -> HandlerResult<()> {
        let input_args = RemoveContainerArgs {
            container: container,
        };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "RemoveContainer",
            &serialize(input_args)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    pub fn remove_object(&self, blob: Blob) -> HandlerResult<()> {
        let input_args = RemoveObjectArgs { blob: blob };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "RemoveObject",
            &serialize(input_args)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    pub fn list_objects(&self, container: Container) -> HandlerResult<BlobList> {
        let input_args = ListObjectsArgs {
            container: container,
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

    pub fn upload_chunk(&self, chunk: FileChunk) -> HandlerResult<()> {
        let input_args = UploadChunkArgs { chunk: chunk };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "UploadChunk",
            &serialize(input_args)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    pub fn start_download(&self, request: StreamRequest) -> HandlerResult<()> {
        let input_args = StartDownloadArgs { request: request };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "StartDownload",
            &serialize(input_args)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    pub fn start_upload(&self, blob: FileChunk) -> HandlerResult<()> {
        let input_args = StartUploadArgs { blob: blob };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "StartUpload",
            &serialize(input_args)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    pub fn receive_chunk(&self, chunk: FileChunk) -> HandlerResult<()> {
        let input_args = ReceiveChunkArgs { chunk: chunk };
        host_call(
            &self.binding,
            "wasmcloud:blobstore",
            "ReceiveChunk",
            &serialize(input_args)?,
        )
        .map(|_vec| ())
        .map_err(|e| e.into())
    }

    pub fn get_object_info(&self, blob: Blob) -> HandlerResult<Blob> {
        let input_args = GetObjectInfoArgs { blob: blob };
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
    pub fn register_create_container(f: fn(Container) -> HandlerResult<Container>) {
        *CREATE_CONTAINER.write().unwrap() = Some(f);
        register_function(&"CreateContainer", create_container_wrapper);
    }
    pub fn register_remove_container(f: fn(Container) -> HandlerResult<()>) {
        *REMOVE_CONTAINER.write().unwrap() = Some(f);
        register_function(&"RemoveContainer", remove_container_wrapper);
    }
    pub fn register_remove_object(f: fn(Blob) -> HandlerResult<()>) {
        *REMOVE_OBJECT.write().unwrap() = Some(f);
        register_function(&"RemoveObject", remove_object_wrapper);
    }
    pub fn register_list_objects(f: fn(Container) -> HandlerResult<BlobList>) {
        *LIST_OBJECTS.write().unwrap() = Some(f);
        register_function(&"ListObjects", list_objects_wrapper);
    }
    pub fn register_upload_chunk(f: fn(FileChunk) -> HandlerResult<()>) {
        *UPLOAD_CHUNK.write().unwrap() = Some(f);
        register_function(&"UploadChunk", upload_chunk_wrapper);
    }
    pub fn register_start_download(f: fn(StreamRequest) -> HandlerResult<()>) {
        *START_DOWNLOAD.write().unwrap() = Some(f);
        register_function(&"StartDownload", start_download_wrapper);
    }
    pub fn register_start_upload(f: fn(FileChunk) -> HandlerResult<()>) {
        *START_UPLOAD.write().unwrap() = Some(f);
        register_function(&"StartUpload", start_upload_wrapper);
    }
    pub fn register_receive_chunk(f: fn(FileChunk) -> HandlerResult<()>) {
        *RECEIVE_CHUNK.write().unwrap() = Some(f);
        register_function(&"ReceiveChunk", receive_chunk_wrapper);
    }
    pub fn register_get_object_info(f: fn(Blob) -> HandlerResult<Blob>) {
        *GET_OBJECT_INFO.write().unwrap() = Some(f);
        register_function(&"GetObjectInfo", get_object_info_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static! {
    static ref CREATE_CONTAINER: RwLock<Option<fn(Container) -> HandlerResult<Container>>> =
        RwLock::new(None);
    static ref REMOVE_CONTAINER: RwLock<Option<fn(Container) -> HandlerResult<()>>> =
        RwLock::new(None);
    static ref REMOVE_OBJECT: RwLock<Option<fn(Blob) -> HandlerResult<()>>> = RwLock::new(None);
    static ref LIST_OBJECTS: RwLock<Option<fn(Container) -> HandlerResult<BlobList>>> =
        RwLock::new(None);
    static ref UPLOAD_CHUNK: RwLock<Option<fn(FileChunk) -> HandlerResult<()>>> = RwLock::new(None);
    static ref START_DOWNLOAD: RwLock<Option<fn(StreamRequest) -> HandlerResult<()>>> =
        RwLock::new(None);
    static ref START_UPLOAD: RwLock<Option<fn(FileChunk) -> HandlerResult<()>>> = RwLock::new(None);
    static ref RECEIVE_CHUNK: RwLock<Option<fn(FileChunk) -> HandlerResult<()>>> =
        RwLock::new(None);
    static ref GET_OBJECT_INFO: RwLock<Option<fn(Blob) -> HandlerResult<Blob>>> = RwLock::new(None);
}

#[cfg(feature = "guest")]
fn create_container_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<CreateContainerArgs>(input_payload)?;
    let lock = CREATE_CONTAINER.read().unwrap().unwrap();
    let result = lock(input.container_id)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn remove_container_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<RemoveContainerArgs>(input_payload)?;
    let lock = REMOVE_CONTAINER.read().unwrap().unwrap();
    let result = lock(input.container)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn remove_object_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<RemoveObjectArgs>(input_payload)?;
    let lock = REMOVE_OBJECT.read().unwrap().unwrap();
    let result = lock(input.blob)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn list_objects_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<ListObjectsArgs>(input_payload)?;
    let lock = LIST_OBJECTS.read().unwrap().unwrap();
    let result = lock(input.container)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn upload_chunk_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<UploadChunkArgs>(input_payload)?;
    let lock = UPLOAD_CHUNK.read().unwrap().unwrap();
    let result = lock(input.chunk)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn start_download_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<StartDownloadArgs>(input_payload)?;
    let lock = START_DOWNLOAD.read().unwrap().unwrap();
    let result = lock(input.request)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn start_upload_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<StartUploadArgs>(input_payload)?;
    let lock = START_UPLOAD.read().unwrap().unwrap();
    let result = lock(input.blob)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn receive_chunk_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<ReceiveChunkArgs>(input_payload)?;
    let lock = RECEIVE_CHUNK.read().unwrap().unwrap();
    let result = lock(input.chunk)?;
    Ok(serialize(result)?)
}

#[cfg(feature = "guest")]
fn get_object_info_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<GetObjectInfoArgs>(input_payload)?;
    let lock = GET_OBJECT_INFO.read().unwrap().unwrap();
    let result = lock(input.blob)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CreateContainerArgs {
    #[serde(rename = "container")]
    pub container_id: Container,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RemoveContainerArgs {
    #[serde(rename = "container")]
    pub container: Container,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RemoveObjectArgs {
    #[serde(rename = "blob")]
    pub blob: Blob,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ListObjectsArgs {
    #[serde(rename = "container")]
    pub container: Container,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct UploadChunkArgs {
    #[serde(rename = "chunk")]
    pub chunk: FileChunk,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StartDownloadArgs {
    #[serde(rename = "request")]
    pub request: StreamRequest,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StartUploadArgs {
    #[serde(rename = "blob")]
    pub blob: FileChunk,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct ReceiveChunkArgs {
    #[serde(rename = "chunk")]
    pub chunk: FileChunk,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GetObjectInfoArgs {
    #[serde(rename = "blob")]
    pub blob: Blob,
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
    pub chunk_bytes: Option<Vec<u8>>,
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
