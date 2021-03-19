# wasmCloud Logging Actor Interface

This actor provides an abstraction over the `wasmcloud:logging` contract. This
allows actors to use normal logging notation (like `info`, `warn`, `error`, etc)
to write logs from within the actor.

```go
package main

import (
	core "github.com/wasmcloud/actor-interfaces/actor-core/go"
	log "github.com/wasmcloud/actor-interfaces/logging/go"
)

var logger *log.Host

func main() {
	logger = log.NewHost("default")

	core.Handlers{
		HealthRequest: healthRequest,
	}.Register()

	log.Handlers{
		WriteLog: writeLog,
	}.Register()

}

func healthRequest(request core.HealthCheckRequest) (core.HealthCheckResponse, error) {
	return core.HealthCheckResponse{
		Healthy: true,
	}, nil
}

func writeLog(target string, level string, text string) error {
	logger.WriteLog(target, level, text)
	return nil
}
```
