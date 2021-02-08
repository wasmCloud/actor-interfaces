# Actor Interfaces

In this repository you'll find reusable packages/modules that are intended to be used by actors written in languages like **Go**, **Rust**, **Zig**, and **AssemblyScript**. Each of the reusable modules (HTTP server, HTTP client, Message Broker, Key-Value Store, etc) are thin veneers wrapping code generated from a **WIDL** schema.

## First-Party Interfaces

Here is a list of the wasmCloud-supported actor interfaces in this repository. Click the green check mark for the supported interface you're interested in.

| Interface | 🦀 Rust | TinyGo | AssemblyScript |
| --- | :---: | :---: | :---: |
| [Core](./actor-core/core.widl) | [✅](./actor-core/rust/README.md) | ⛔ | [✅](./actor-core/assemblyscript/README.md) |
| [HTTP Server](./http-server/http.widl) | [✅](./http-server/rust/README.md) | ⛔ | [✅](./http-server/assemblyscript/README.md) |
| [HTTP Client](./http-client/http_client.widl) |[✅](./http-client/rust/README.md)| ⛔ | ⛔ |
| [Key-Value Store](./keyvalue/keyvalue.widl) | [✅](./keyvalue/rust/README.md) | ⛔ | [✅](./keyvalue/assemblyscript/README.md) |
| [Messaging](./messaging//messaging.widl) | [✅](./messaging/rust/README.md) | ⛔ | ⛔ |
| [Telnet](./telnet/telnet.widl) | [✅](./telnet/rust/README.md) | ⛔ | ⛔ |
| [GraphDB](./graphdb/graphdb.widl) | [✅](./graphdb/rust/README.md) | ⛔ | ⛔ |
| [Blob Store](./blobstore/blobstore.widl) | [✅](./blobstore/rust/README.md) | ⛔ | ⛔ |
| [Event Streams](./eventstreams/eventstreams.widl) | [✅](./eventstreams/rust/README.md) | ⛔ | ⛔ |
| [Logging](./logging/logging.widl) | [✅](./logging/rust/README.md) | ⛔ | ⛔ |
| [Extras](./extras/extras.widl) | [✅](./extras/rust/README.md) | ⛔ | ⛔ |
