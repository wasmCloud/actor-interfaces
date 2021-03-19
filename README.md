# Actor Interfaces

In this repository you'll find reusable packages/modules that are intended to be used by actors written in languages like **Go**, **Rust**, **Zig**, and **AssemblyScript**. Each of the reusable modules (HTTP server, HTTP client, Message Broker, Key-Value Store, etc) are thin veneers wrapping code generated from a **WIDL** schema.

## First-Party Interfaces

Here is a list of the wasmCloud-supported actor interfaces in this repository. The badges link to the published interfaces on their appropriate package manager.

| Interface | 🦀 Rust | TinyGo | AssemblyScript |
| --- | :---: | :---: | :---: |
| [Core](./actor-core/core.widl) | [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-core)](https://crates.io/crates/wasmcloud-actor-core) | [![GitHub go.mod Go version (subdirectory of monorepo)](https://img.shields.io/github/go-mod/go-version/wasmcloud/actor-interfaces?filename=actor-core%2Fgo%2Fgo.mod)](https://pkg.go.dev/github.com/wasmcloud/actor-interfaces/actor-core/go) | [![npm](https://img.shields.io/npm/v/@wasmcloud/actor-core?color=green)](https://www.npmjs.com/package/@wasmcloud/actor-core) |
| [HTTP Server](./http-server/http.widl) | [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-http-server)](https://crates.io/crates/wasmcloud-actor-http-server) | [![GitHub go.mod Go version (subdirectory of monorepo)](https://img.shields.io/github/go-mod/go-version/wasmcloud/actor-interfaces?filename=http-server%2Fgo%2Fgo.mod)](https://pkg.go.dev/github.com/wasmcloud/actor-interfaces/http-server/go) | [![npm](https://img.shields.io/npm/v/@wasmcloud/actor-http-server?color=green)](https://www.npmjs.com/package/@wasmcloud/actor-http-server) |
| [HTTP Client](./http-client/http_client.widl) | [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-http-client)](https://crates.io/crates/wasmcloud-actor-http-client) | ⛔ | ⛔ |
| [Key-Value Store](./keyvalue/keyvalue.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-keyvalue)](https://crates.io/crates/wasmcloud-actor-keyvalue)  | ⛔ | [![npm](https://img.shields.io/npm/v/@wasmcloud/actor-keyvalue?color=green)](https://www.npmjs.com/package/@wasmcloud/actor-keyvalue) |
| [Messaging](./messaging//messaging.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-messaging)](https://crates.io/crates/wasmcloud-actor-messaging)  | ⛔ | ⛔ |
| [Telnet](./telnet/telnet.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-telnet)](https://crates.io/crates/wasmcloud-actor-telnet)  | ⛔ | ⛔ |
| [GraphDB](./graphdb/graphdb.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-graphdb)](https://crates.io/crates/wasmcloud-actor-graphdb)  | ⛔ | ⛔ |
| [Blob Store](./blobstore/blobstore.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-blobstore)](https://crates.io/crates/wasmcloud-actor-blobstore)  | ⛔ | ⛔ |
| [Event Streams](./eventstreams/eventstreams.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-eventstreams)](https://crates.io/crates/wasmcloud-actor-eventstreams)  | ⛔ | ⛔ |
| [Logging](./logging/logging.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-logging)](https://crates.io/crates/wasmcloud-actor-logging)  | [![GitHub go.mod Go version (subdirectory of monorepo)](https://img.shields.io/github/go-mod/go-version/wasmcloud/actor-interfaces?filename=logging%2Fgo%2Fgo.mod)](https://pkg.go.dev/github.com/wasmcloud/actor-interfaces/logging/go) | ⛔ |
| [Extras](./extras/extras.widl) |  [![Crates.io](https://img.shields.io/crates/v/wasmcloud-actor-extras)](https://crates.io/crates/wasmcloud-actor-extras)  | ⛔ | ⛔ |
