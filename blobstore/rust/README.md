# wasmCloud Blob Store Actor Interface

This crate provides wasmCloud actors with an interface to the blobstore capability provider.
Actors using this interface must have the claim `wasmcloud:blobstore` in order to have
permission to communicate with the store.

This generic protocol can be used to support capability providers like local blob storage,
Amazon S3, Azure blob storage, Google blob storage, and more.

Example:
```rust
extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_blobstore as blobstore;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_core::serialize;
use blobstore::*;
use serde_json::json;
use log::{error, info};

#[no_mangle]
pub fn wapc_init() {
    http::Handlers::register_handle_request(download_poem);
    blobstore::Handlers::register_receive_chunk(handle_chunk);
    actor_core::Handlers::register_health_request(health);
}

/// Start the download of a blob (poem). Chunks will be streamed after download begins
fn download_poem(req: http::Request) -> HandlerResult<http::Response> {
    let stream_request = StreamRequest {
        id: req.path,
        container: Container::new("photos".to_string()),
        chunk_size: 4096,
        context: None
    };
    // replace `start_download` with `blobstore::default().start_download`
    match start_download(stream_request) {
        Ok(_) => Ok(http::Response::ok()),
        Err(_) => Err("Failed to initiate download of chunk".into())
    }
}

/// Handle the incoming chunk as a poem "verse" and log the result
/// Note that these chunks can be received out of order, so the poem
/// in this case might be displayed in a different order.
fn handle_chunk(chunk: FileChunk) -> HandlerResult<()> {
    let verse = String::from_utf8(chunk.chunk_bytes)?;
    info!("Poem {} part {}:\n{}", chunk.id, chunk.sequence_no, verse);
    Ok(())
}

fn health(_: actor_core::HealthCheckRequest) -> HandlerResult<actor_core::HealthCheckResponse> {
  Ok(actor_core::HealthCheckResponse::healthy())   
}

```
