# wasmcloud Actor Core - Derive Crate

This crate contains the macro definition for `actor::init`, which is an annotation macro you can place on your actor initialization function. You will never need to use this crate directly, but will instead use [wasmcloud-actor-core](../wasmcloud-actor-core/README.md). The only reason this crate is isolated on its own is because it uses the `proc_macro = true` option inside the `[lib]` section of `Cargo.toml`, and that option requires a standalone crate.
