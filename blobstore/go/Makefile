.PHONY: all deps codegen build clean doc test

all: deps codegen build

deps:

codegen:
	wapc generate codegen.yaml

clean:
	rm -Rf build

doc:

test:
	go test --count=1 ./pkg/...
