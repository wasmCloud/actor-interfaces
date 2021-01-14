# Actor Interfaces

In this repository you'll find reusable packages/modules that are intended to be used by actors written in languages like **Go**, **Rust**, **Zig**, and **AssemblyScript**. Each of the reusable modules (HTTP server, HTTP client, Message Broker, Key-Value Store, etc) are thin veneers wrapping code generated from a **WIDL** schema.

## First-Party Interfaces

Here is a list of the wasmCloud-supported actor interfaces in this repository. Click the green check mark for the supported interface you're interested in.

| Interface | 🦀 Rust | TinyGo | AssemblyScript |
| --- | :---: | :---: | :---: |
| [Core](./schemas/core.widl) | [✅](./rust/actor-core/README.md) |  | |
| [HTTP Server](./schemas/http.widl) | [✅](./rust/http-server/README.md) | ⛔ | ⛔ |
| [HTTP Client](./schemas/http.widl) | [✅](./rust/http-client/README.md) | ⛔|⛔|
| [Key-Value Store](./schemas/keyvalue.widl) | [✅](./rust/keyvalue/README.md) | ⛔ | ⛔ |
| [Messaging](./schemas/messaging.widl) | [✅](./rust/messaging/README.md) | ⛔ | ⛔ |
| [Telnet](./schemas/telnet.widl) | [✅](./rust/telnet/README.md) | ⛔ | ⛔ |
| [ GraphDB](./schemas/graphdb.widl) | [✅](./rust/graphdb/README.md) | ⛔ | ⛔ |
| [Blob Store](./schemas/blobstore.widl) | [✅](./rust/blobstore/README.md) | ⛔ | ⛔ |
| [Event Streams](./schemas/eventstreams.widl) | [✅](./rust/eventstreams/README.md) | ⛔ | ⛔ |
| [Logging](./schemas/logging.widl) | [✅](./rust/logging/README.md) | ⛔ | ⛔ |
