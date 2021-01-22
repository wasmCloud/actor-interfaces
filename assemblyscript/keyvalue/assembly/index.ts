/**
  * wasmCloud Key-Value Store Actor Interface
  * 
  * This crate provides wasmCloud actors with an interface to the key-value capability provider. Actors using this
  * interface must have the claim `wasmcloud:keyvalue` in order to have permission to communicate with the store.
  * 
  * The key-value provider is one-way, and only accepts host calls from the actor. This provider does _not_
  * deliver messages to actors.
 */

// import {
//   GetResponse,
//   AddResponse,
//   DelResponse,
//   ListRangeResponse,
//   ListResponse,
//   SetResponse,
//   SetOperationResponse,
//   SetQueryResponse,
//   Handlers,
// } from "./module";

// export function wapc_init(): void {
//   Handlers.registerGet(Get);
//   Handlers.registerAdd(Add);
//   Handlers.registerSet(Set);
//   Handlers.registerDel(Del);
//   Handlers.registerClear(Clear);
//   Handlers.registerRange(Range);
//   Handlers.registerPush(Push);
//   Handlers.registerListItemDelete(ListItemDelete);
//   Handlers.registerSetAdd(SetAdd);
//   Handlers.registerSetRemove(SetRemove);
//   Handlers.registerSetUnion(SetUnion);
//   Handlers.registerSetIntersection(SetIntersection);
//   Handlers.registerSetQuery(SetQuery);
//   Handlers.registerKeyExists(KeyExists);
// }

// function Get(key: string): GetResponse {
//   return new GetResponse(); // TODO: Provide implementation.
// }

// function Add(key: string, value: i32): AddResponse {
//   return new AddResponse(); // TODO: Provide implementation.
// }

// function Set(key: string, value: string, expires: i32): SetResponse {
//   return new SetResponse(); // TODO: Provide implementation.
// }

// function Del(key: string): DelResponse {
//   return new DelResponse(); // TODO: Provide implementation.
// }

// function Clear(key: string): DelResponse {
//   return new DelResponse(); // TODO: Provide implementation.
// }

// function Range(key: string, start: i32, stop: i32): ListRangeResponse {
//   return new ListRangeResponse(); // TODO: Provide implementation.
// }

// function Push(key: string, value: string): ListResponse {
//   return new ListResponse(); // TODO: Provide implementation.
// }

// function ListItemDelete(key: string, value: string): ListResponse {
//   return new ListResponse(); // TODO: Provide implementation.
// }

// function SetAdd(key: string, value: string): SetOperationResponse {
//   return new SetOperationResponse(); // TODO: Provide implementation.
// }

// function SetRemove(key: string, value: string): SetOperationResponse {
//   return new SetOperationResponse(); // TODO: Provide implementation.
// }

// function SetUnion(keys: Array<string>): SetQueryResponse {
//   return new SetQueryResponse(); // TODO: Provide implementation.
// }

// function SetIntersection(keys: Array<string>): SetQueryResponse {
//   return new SetQueryResponse(); // TODO: Provide implementation.
// }

// function SetQuery(key: string): SetQueryResponse {
//   return new SetQueryResponse(); // TODO: Provide implementation.
// }

// function KeyExists(key: string): GetResponse {
//   return new GetResponse(); // TODO: Provide implementation.
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
