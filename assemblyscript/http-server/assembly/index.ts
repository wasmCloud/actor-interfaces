/**
 * HTTP Server wasmCloud Actor Interface
 * 
 * This module provides wasmCloud actors with an interface to the HTTP Server capability provider. Actors using this
 * interface must have the claim `wasmcloud:httpserver` in order to have permission to handle requests, and they
 * must have an active, configured binding to an HTTP Server capability provider.
 *
 * The HTTP Server provider is one-way, and only delivers messages to actors. Actors cannot make host calls
 * to this provider.
 */

export * from "./module";