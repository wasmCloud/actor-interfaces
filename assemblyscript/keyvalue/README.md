# wasmCloud Key-Value Store Actor Interface

This module provides wasmCloud actors with an interface to the key-value capability provider. Actors using this
interface must have the claim `wasmcloud:keyvalue` in order to have permission to communicate with the store.

The key-value provider is one-way, and only accepts host calls from the actor. This provider does _not_
deliver messages to actors.
