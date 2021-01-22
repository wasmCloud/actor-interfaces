/**
 * Core wasmCloud Actor Interface
 * All wasmCloud actors must respond to the core health request message.
 */

// import {
//   CapabilityConfiguration,
//   HealthCheckRequest,
//   HealthCheckResponse,
//   Handlers,
// } from "./module";

// export function wapc_init(): void {
//   Handlers.registerHealthRequest(HealthRequest);
// }

// function HealthRequest(request: HealthCheckRequest): HealthCheckResponse {
//   return new HealthCheckResponse(); // TODO: Provide implementation.
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
