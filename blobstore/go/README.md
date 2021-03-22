# wasmCloud Blobstore Actor Interface
 
This crate provides wasmCloud actors with an interface to the blobstore capability provider.
Actors using this interface must have the claim `wasmcloud:blobstore` in order to have
permission to communicate with the store.

This generic protocol can be used to support capability providers like local blob storage,
Amazon S3, Azure blob storage, Google blob storage, and more.

## Sample Actor
```go
package main

import (
	"strings"
	core "github.com/wasmcloud/actor-interfaces/actor-core/go"
	httpserver "github.com/wasmcloud/actor-interfaces/http-server/go"
	blob "github.com/wasmcloud/actor-interfaces/blobstore/go"
)

func main() {
	core.Handlers{HealthRequest: healthRequest}.Register()
	httpserver.Handlers{HandleRequest: handleRequest}.Register()
  blob.Handlers{ReceiveChunk: receiveChunk}.Register()
}

func receiveChunk(chunk blob.FileChunk) error {
  // ReceiveChunk is used to process incoming file chunks, as downloading an entire file
  // may exceed the memory capacity of the WebAssembly module.
  // 
  // This is effectively streaming a file download to the module, so you must use
  // the chunk `Context`, `TotalBytes` and `ChunkBytes` fields to determine
  // when you have received all of the bytes for a file.
  return nil
}

func handleRequest(request httpserver.Request) (httpserver.Response, error) {
	switch method := request.Method; method {
	case "GET":
        return downloadImage()
	case "POST":
		return uploadImage(request.Path, request.Body)
	default:
		return httpserver.Response{
			StatusCode: 400,
			Status:     "Bad Request",
			Body:       nil,
		}, nil
	}
}

var blobStore *blob.Host
func downloadImage() (httpserver.Response, error) {
	blobStore = blob.NewHost("default")
	blobStore.StartDownload("myblob", "folder", 512, nil)
	return httpserver.Response{
		StatusCode: 200,
		Status:     "OK",
		Body:       []byte("Download started"),
	}, nil
}

func uploadImage(path string, imageBytes []byte) (httpserver.Response, error) {
    blobStore = blob.NewHost("default")
    container := blob.Container {
        ID: "folder",
    }
    image := blob.FileChunk {
        SequenceNo: 0,
        Container: container,
        ID: strings.ReplaceAll(path, "/", ""),
        TotalBytes: uint64(len(imageBytes)),
        ChunkSize: uint64(len(imageBytes)),
        Context: nil,
        ChunkBytes: imageBytes,
    }
    blobStore.StartUpload(image)
    blobStore.UploadChunk(image)
    return httpserver.Response{
        StatusCode: 200,
        Status:     "OK",
        Body:       []byte("Uploaded Successfully"),
    }, nil
}

func healthRequest(request core.HealthCheckRequest) (core.HealthCheckResponse, error) {
	return core.HealthCheckResponse{
		Healthy: true,
	}, nil
}
```