/**
 * Blobstore wasmCloud Actor Interface
 * 
 * This module provides wasmCloud actors with an interface to blobstore capability providera. Actors using this
 * interface must have the claim `wasmcloud:blobstore` in order to have permission to handle requests.
 * 
 * Example providers that satisfy the `wasmcloud:blobstore` contract are wasmcloud-fs and wasmcloud-s3, a filesystem
 * and AWS S3 implementation (respectively).
 */

// Export shared types for actor use
export * from './module';

// TODO: Sample actor below
// import { handleCall, handleAbort } from "@wapc/as-guest";
// import {
//   FileChunk,
//   Handlers,
// } from "./module";

// export function wapc_init(): void {
//   Handlers.registerReceiveChunk(ReceiveChunk);
// }

// function ReceiveChunk(chunk: FileChunk): void {
//   // TODO: Provide implementation.
// }

// // Boilerplate code for waPC.  Do not remove.
// export function __guest_call(operation_size: usize, payload_size: usize): bool {
//   return handleCall(operation_size, payload_size);
// }

// // Abort function
// function abort(
//   message: string | null,
//   fileName: string | null,
//   lineNumber: u32,
//   columnNumber: u32
// ): void {
//   handleAbort(message, fileName, lineNumber, columnNumber);
// }
