# wasmCloud Core Actor Interface

This is the actor interface for the `wasmcloud:core` capability contract ID. This interface 
is required by all actors. Using this interface your actor can implement a handler for
the health check message.

```go
package main

import (
	core "github.com/wasmcloud/actor-interfaces/actor-core/go"
)

func main() {
	core.Handlers{
		HealthRequest: healthRequest,
	}.Register()
}

func healthRequest(request core.HealthCheckRequest) (core.HealthCheckResponse, error) {
	return core.HealthCheckResponse{
        healthy: true
    }, nil // TODO: Provide implementation.
}
```
