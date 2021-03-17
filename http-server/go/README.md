# wasmCloud HTTP Server Actor Interface

This actor interface provides wasmCloud actors with an interface to the HTTP Server capability provider. Actors using this
interface must have the claim `wasmcloud:httpserver` in order to have permission to handle requests, and they
must have an active, configured link to an HTTP Server capability provider.

The HTTP Server provider is one-way, and only delivers messages to actors. Actors cannot make host calls
to this provider.

The following is an example of how to use this provider:

```go
package main

import (
	core "github.com/wasmcloud/actor-interfaces/actor-core/go"
	httpserver "github.com/wasmcloud/actor-interfaces/http-server/go"
)

func main() {
	core.Handlers{HealthRequest: healthRequest}.Register()
	httpserver.Handlers{HandleRequest: handleRequest}.Register()
}

func healthRequest(request core.HealthCheckRequest) (core.HealthCheckResponse, error) {
	return core.HealthCheckResponse{
		Healthy: true,
	}, nil
}

func handleRequest(request httpserver.Request) (httpserver.Response, error) {
	switch method := request.Method; method {
	case "GET":
		return httpserver.Response{
			StatusCode: 200,
			Status:     "OK",
			Body:       []byte("GOT GET"),
		}, nil
	case "POST":
		return httpserver.Response{
			StatusCode: 200,
			Status:     "OK",
			Body:       []byte("GOT POST"),
		}, nil
	default:
		return httpserver.Response{
			StatusCode: 400,
			Status:     "Bad Request",
			Body:       nil,
		}, nil
	}
}
```
