/**
 * Blobstore wasmCloud Actor Interface
 * 
 * This module provides wasmCloud actors with an interface to blobstore capability provider. Actors using this
 * interface must have the claim `wasmcloud:blobstore` in order to have permission to handle requests.
 * 
 * Example providers that satisfy the `wasmcloud:blobstore` contract are wasmcloud-fs and wasmcloud-s3, a filesystem
 * and AWS S3 implementation (respectively).
 */

// Export shared types for actor use
export * from './module';