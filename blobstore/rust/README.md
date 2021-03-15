[![crates.io](https://img.shields.io/crates/v/wasmcloud-actor-blobstore.svg)](https://crates.io/crates/wasmcloud-actor-blobstore)
![Rust](https://img.shields.io/github/workflow/status/wasmcloud/actor-interfaces/Blobstore)
![license](https://img.shields.io/crates/l/wasmcloud-actor-blobstore.svg)
[![documentation](https://docs.rs/wasmcloud-actor-blobstore/badge.svg)](https://docs.rs/wasmcloud-actor-blobstore)

# wasmCloud Blob Store Actor Interface

This crate provides wasmCloud actors with an interface to the blobstore capability provider.
Actors using this interface must have the claim `wasmcloud:blobstore` in order to have
permission to communicate with the store.

This generic protocol can be used to support capability providers like local blob storage,
Amazon S3, Azure blob storage, Google blob storage, and more.

## Example

```rust
extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_blobstore as blobstore;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_core as actor;
use actor::{serialize, init};
use blobstore::*;
use serde_json::json;
use log::{error, info};

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(download_poem);
    blobstore::Handlers::register_receive_chunk(handle_chunk);   
}

fn download_poem(req: http::Request) -> HandlerResult<http::Response> {    
    match blobstore::default().start_download(req.path, "poems".to_string(), 4096, None) {
        Ok(_) => Ok(http::Response::ok()),
        Err(_) => Err("Failed to initiate download of chunk".into())
    }
}

 /// Handle the incoming chunk as a poem "verse" and log the result
 /// Note that these chunks can be received out of order, so the poem
 /// in this case might be displayed in a different order and could look
 /// funny if the verse continues across a chunk boundary
 fn handle_chunk(chunk: FileChunk) -> HandlerResult<()> {
     let verse = String::from_utf8(chunk.chunk_bytes)?;
     info!("Poem {} part {}:\n{}", chunk.id, chunk.sequence_no, verse);
     Ok(())
 }


```
