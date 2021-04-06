.PHONY: all deps codegen build clean doc test

all: deps codegen build

deps:

codegen:
	wapc generate codegen.yaml

#build:
#	mkdir -p build
#	tinygo build -o build/tinygo.wasm -target wasm -no-debug pkg/main.go

clean:
	rm -Rf build

doc:

test:
	go test --count=1 ./pkg/...
