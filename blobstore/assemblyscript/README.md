# wasmCloud Blobstore Actor Interface
 
This crate provides wasmCloud actors with an interface to the blobstore capability provider.
Actors using this interface must have the claim `wasmcloud:blobstore` in order to have
permission to communicate with the store.

This generic protocol can be used to support capability providers like local blob storage,
Amazon S3, Azure blob storage, Google blob storage, and more.

## Sample Actor
```typescript
import { Request, Response, ResponseBuilder, Handlers as HTTPHandlers } from "@wasmcloud/actor-http-server";
import { Host as Blob, ContainerBuilder, FileChunk, FileChunkBuilder, Handlers as BlobHandlers } from "../../../../../github.com/wasmcloud/actor-interfaces/blobstore/assemblyscript/assembly/module";
import { HealthCheckResponse, HealthCheckRequest, Handlers as CoreHandlers, HealthCheckResponseBuilder } from "@wasmcloud/actor-core";

export function wapc_init(): void {
  CoreHandlers.registerHealthRequest(HealthCheck);
  HTTPHandlers.registerHandleRequest(HandleRequest);
  BlobHandlers.registerReceiveChunk(ReceiveChunk);
}

function ReceiveChunk(chunk: FileChunk) {
  // ReceiveChunk is used to process incoming file chunks, as downloading an entire file
  // may exceed the memory capacity of the WebAssembly module.
  // 
  // This is effectively streaming a file download to the module, so you must use
  // the chunk `context`, `totalBytes` and `chunkBytes` fields to determine
  // when you have received all of the bytes for a file.
}

function HandleRequest(request: Request): Response {
  if (request.method == "GET") {
    return download_image();
  } else if (request.method == "POST") {
    return upload_image(request.path, request.body);
  } else {
    return new ResponseBuilder()
      .withStatusCode(400)
      .withStatus("Bad Request")
      .withBody(String.UTF8.encode(`method ${request.method} not supported`, true))
      .build();
  }
}

function download_image(): Response {
  const blobstore = new Blob("default");
  blobstore.StartDownload("myblob", "folder", 512, null);
  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(String.UTF8.encode("download started", true))
    .build();
}

function upload_image(path: string, image_bytes: ArrayBuffer): Response {
  const blobstore = new Blob("default");
  let container = new ContainerBuilder().withId("folder").build();
  let image = new FileChunkBuilder()
    .withSequenceNo(0)
    .withContainer(container)
    .withId(path.replaceAll("/", ""))
    .withTotalBytes(image_bytes.byteLength)
    .withChunkSize(image_bytes.byteLength)
    .withChunkBytes(image_bytes)
    .build();
  blobstore.StartUpload(image);
  blobstore.UploadChunk(image);
  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(String.UTF8.encode("upload successful", true))
    .build();
}


function HealthCheck(request: HealthCheckRequest): HealthCheckResponse {
  return new HealthCheckResponseBuilder().withHealthy(true).withMessage("Healthy").build();
}

// Boilerplate code for waPC.  Do not remove.
import { handleCall, handleAbort } from "@wapc/as-guest";

export function __guest_call(operation_size: usize, payload_size: usize): bool {
  return handleCall(operation_size, payload_size);
}

// Abort function
function abort(
  message: string | null,
  fileName: string | null,
  lineNumber: u32,
  columnNumber: u32
): void {
  handleAbort(message, fileName, lineNumber, columnNumber);
}

```