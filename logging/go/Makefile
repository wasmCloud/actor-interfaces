.PHONY: all deps codegen clean doc test

all: deps codegen

deps:

codegen:
	wapc generate codegen.yaml

clean:
	rm -Rf build

doc:

test:
	go test --count=1 ./pkg/...
