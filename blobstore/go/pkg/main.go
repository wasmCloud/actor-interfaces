package main

import (
	"github.com/myorg/mymodule/pkg/module"
)

func main() {
	module.Handlers{
		ReceiveChunk:    ReceiveChunk,
	}.Register()
}
func ReceiveChunk(chunk module.FileChunk) error {
	return nil // TODO: Provide implementation.
}
