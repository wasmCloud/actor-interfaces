/**
  * wasmCloud Key-Value Store Actor Interface
  * 
  * This module provides wasmCloud actors with an interface to the key-value capability provider. Actors using this
  * interface must have the claim `wasmcloud:keyvalue` in order to have permission to communicate with the store.
  * 
  * The key-value provider is one-way, and only accepts host calls from the actor. This provider does _not_
  * deliver messages to actors.
 */

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
