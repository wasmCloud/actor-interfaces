package main

import (
	"github.com/myorg/mymodule/pkg/module"
)

func main() {
	module.Handlers{
		HealthRequest: HealthRequest,
	}.Register()
}

func HealthRequest(request module.HealthCheckRequest) (module.HealthCheckResponse, error) {
	return module.HealthCheckResponse{}, nil // TODO: Provide implementation.
}
