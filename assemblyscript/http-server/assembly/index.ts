/**
 * HTTP Server wasmCloud Actor Interface
 * 
 * This crate provides wasmCloud actors with an interface to the HTTP Server capability provider. Actors using this
 * interface must have the claim `wascc:http_server` in order to have permission to handle requests, and they
 * must have an active, configured binding to an HTTP Server capability provider.
 *
 * The HTTP Server provider is one-way, and only delivers messages to actors. Actors cannot make host calls
 * to this provider.
 */


// import { Request, Response, Handlers } from "./module";

// export function wapc_init(): void {
//   Handlers.registerHandleRequest(HandleRequest);
// }

// function HandleRequest(request: Request): Response {
//   return new Response(); // TODO: Provide implementation.
// }

// Boilerplate code for waPC.  Do not remove.
import { handleCall, handleAbort } from "@wapc/as-guest";

export function __guest_call(operation_size: usize, payload_size: usize): bool {
  return handleCall(operation_size, payload_size);
}

// Abort function
function abort(
  message: string | null,
  fileName: string | null,
  lineNumber: u32,
  columnNumber: u32
): void {
  handleAbort(message, fileName, lineNumber, columnNumber);
}