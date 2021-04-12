import { Decoder, Writer, Encoder, Sizer, Codec } from "@wapc/as-msgpack";

import { hostCall } from "@wapc/as-guest";
export class Host {
  binding: string;

  constructor(binding: string = "default") {
    this.binding = binding;
  }

  // Create a container in a blobstore. Returns the container created if successful
  createContainer(container: Container): Container {
    const inputArgs = new CreateContainerArgs();
    inputArgs.container = container;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "CreateContainer",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return Container.decode(decoder);
  }

  // Remove a container from a blobstore
  removeContainer(container: Container): void {
    const inputArgs = new RemoveContainerArgs();
    inputArgs.container = container;
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "RemoveContainer",
      inputArgs.toBuffer()
    );
  }

  // Remove an object from a blobstore
  removeObject(blob: Blob): void {
    const inputArgs = new RemoveObjectArgs();
    inputArgs.blob = blob;
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "RemoveObject",
      inputArgs.toBuffer()
    );
  }

  // Returns a list of blobs that are present in the specified container
  listObjects(container: Container): BlobList {
    const inputArgs = new ListObjectsArgs();
    inputArgs.container = container;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "ListObjects",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return BlobList.decode(decoder);
  }

  // Upload a file chunk to a blobstore, which may only be part of a full file. This
  // must be called AFTER the StartUpload operation. Chunks should be small, as
  // memory over a few megabytes may exceed the wasm memory allocation.
  uploadChunk(chunk: FileChunk): void {
    const inputArgs = new UploadChunkArgs();
    inputArgs.chunk = chunk;
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "UploadChunk",
      inputArgs.toBuffer()
    );
  }

  // Issue a request to start a download from a blobstore. Chunks will be sent to the
  // function that's registered with the ReceiveChunk operation.
  startDownload(request: StreamRequest): void {
    const inputArgs = new StartDownloadArgs();
    inputArgs.request = request;
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "StartDownload",
      inputArgs.toBuffer()
    );
  }

  // Begin the upload process with the first chunk of a full file. Subsequent chunks
  // should be uploaded with the UploadChunk operation.
  startUpload(chunk: FileChunk): void {
    const inputArgs = new StartUploadArgs();
    inputArgs.chunk = chunk;
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "StartUpload",
      inputArgs.toBuffer()
    );
  }

  // Retreives information about a blob
  getObjectInfo(blob: Blob): Blob {
    const inputArgs = new GetObjectInfoArgs();
    inputArgs.blob = blob;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "GetObjectInfo",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return Blob.decode(decoder);
  }
}

import { register } from "@wapc/as-guest";
export class Handlers {
  // Defines a handler for incoming chunks forwarded by a wasmcloud:blobstore
  // provider. Chunks may not be received in order.
  static registerReceiveChunk(handler: (chunk: FileChunk) => void): void {
    ReceiveChunkHandler = handler;
    register("ReceiveChunk", ReceiveChunkWrapper);
  }
}

var ReceiveChunkHandler: (chunk: FileChunk) => void;
function ReceiveChunkWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new ReceiveChunkArgs();
  inputArgs.decode(decoder);
  ReceiveChunkHandler(inputArgs.chunk);
  return new ArrayBuffer(0);
}

export class CreateContainerArgs implements Codec {
  container: Container = new Container();

  static decodeNullable(decoder: Decoder): CreateContainerArgs | null {
    if (decoder.isNextNil()) return null;
    return CreateContainerArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): CreateContainerArgs {
    const o = new CreateContainerArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "container") {
        this.container = Container.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("container");
    this.container.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class RemoveContainerArgs implements Codec {
  container: Container = new Container();

  static decodeNullable(decoder: Decoder): RemoveContainerArgs | null {
    if (decoder.isNextNil()) return null;
    return RemoveContainerArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): RemoveContainerArgs {
    const o = new RemoveContainerArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "container") {
        this.container = Container.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("container");
    this.container.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class RemoveObjectArgs implements Codec {
  blob: Blob = new Blob();

  static decodeNullable(decoder: Decoder): RemoveObjectArgs | null {
    if (decoder.isNextNil()) return null;
    return RemoveObjectArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): RemoveObjectArgs {
    const o = new RemoveObjectArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "blob") {
        this.blob = Blob.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("blob");
    this.blob.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class ListObjectsArgs implements Codec {
  container: Container = new Container();

  static decodeNullable(decoder: Decoder): ListObjectsArgs | null {
    if (decoder.isNextNil()) return null;
    return ListObjectsArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ListObjectsArgs {
    const o = new ListObjectsArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "container") {
        this.container = Container.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("container");
    this.container.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class UploadChunkArgs implements Codec {
  chunk: FileChunk = new FileChunk();

  static decodeNullable(decoder: Decoder): UploadChunkArgs | null {
    if (decoder.isNextNil()) return null;
    return UploadChunkArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): UploadChunkArgs {
    const o = new UploadChunkArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "chunk") {
        this.chunk = FileChunk.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("chunk");
    this.chunk.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class StartDownloadArgs implements Codec {
  request: StreamRequest = new StreamRequest();

  static decodeNullable(decoder: Decoder): StartDownloadArgs | null {
    if (decoder.isNextNil()) return null;
    return StartDownloadArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): StartDownloadArgs {
    const o = new StartDownloadArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "request") {
        this.request = StreamRequest.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("request");
    this.request.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class StartUploadArgs implements Codec {
  chunk: FileChunk = new FileChunk();

  static decodeNullable(decoder: Decoder): StartUploadArgs | null {
    if (decoder.isNextNil()) return null;
    return StartUploadArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): StartUploadArgs {
    const o = new StartUploadArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "chunk") {
        this.chunk = FileChunk.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("chunk");
    this.chunk.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class GetObjectInfoArgs implements Codec {
  blob: Blob = new Blob();

  static decodeNullable(decoder: Decoder): GetObjectInfoArgs | null {
    if (decoder.isNextNil()) return null;
    return GetObjectInfoArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): GetObjectInfoArgs {
    const o = new GetObjectInfoArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "blob") {
        this.blob = Blob.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("blob");
    this.blob.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

export class ReceiveChunkArgs implements Codec {
  chunk: FileChunk = new FileChunk();

  static decodeNullable(decoder: Decoder): ReceiveChunkArgs | null {
    if (decoder.isNextNil()) return null;
    return ReceiveChunkArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ReceiveChunkArgs {
    const o = new ReceiveChunkArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "chunk") {
        this.chunk = FileChunk.decode(decoder);
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("chunk");
    this.chunk.encode(encoder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }
}

// Represents a single chunk that may comprise part of a file or an entire file.
// The fields for sequence number, total bytes and chunk bytes should be used to
// determine the chunk order, as well as the optional context field.
export class FileChunk implements Codec {
  sequenceNo: u64 = 0;
  container: Container = new Container();
  id: string = "";
  totalBytes: u64 = 0;
  chunkSize: u64 = 0;
  context: string | null = null;
  chunkBytes: ArrayBuffer = new ArrayBuffer(0);

  static decodeNullable(decoder: Decoder): FileChunk | null {
    if (decoder.isNextNil()) return null;
    return FileChunk.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): FileChunk {
    const o = new FileChunk();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "sequenceNo") {
        this.sequenceNo = decoder.readUInt64();
      } else if (field == "container") {
        this.container = Container.decode(decoder);
      } else if (field == "id") {
        this.id = decoder.readString();
      } else if (field == "totalBytes") {
        this.totalBytes = decoder.readUInt64();
      } else if (field == "chunkSize") {
        this.chunkSize = decoder.readUInt64();
      } else if (field == "context") {
        if (decoder.isNextNil()) {
          this.context = null;
        } else {
          this.context = decoder.readString();
        }
      } else if (field == "chunkBytes") {
        this.chunkBytes = decoder.readByteArray();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(7);
    encoder.writeString("sequenceNo");
    encoder.writeUInt64(this.sequenceNo);
    encoder.writeString("container");
    this.container.encode(encoder);
    encoder.writeString("id");
    encoder.writeString(this.id);
    encoder.writeString("totalBytes");
    encoder.writeUInt64(this.totalBytes);
    encoder.writeString("chunkSize");
    encoder.writeUInt64(this.chunkSize);
    encoder.writeString("context");
    if (this.context === null) {
      encoder.writeNil();
    } else {
      encoder.writeString(this.context);
    }
    encoder.writeString("chunkBytes");
    encoder.writeByteArray(this.chunkBytes);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): FileChunkBuilder {
    return new FileChunkBuilder();
  }
}

export class FileChunkBuilder {
  instance: FileChunk = new FileChunk();

  withSequenceNo(sequenceNo: u64): FileChunkBuilder {
    this.instance.sequenceNo = sequenceNo;
    return this;
  }

  withContainer(container: Container): FileChunkBuilder {
    this.instance.container = container;
    return this;
  }

  withId(id: string): FileChunkBuilder {
    this.instance.id = id;
    return this;
  }

  withTotalBytes(totalBytes: u64): FileChunkBuilder {
    this.instance.totalBytes = totalBytes;
    return this;
  }

  withChunkSize(chunkSize: u64): FileChunkBuilder {
    this.instance.chunkSize = chunkSize;
    return this;
  }

  withContext(context: string | null): FileChunkBuilder {
    this.instance.context = context;
    return this;
  }

  withChunkBytes(chunkBytes: ArrayBuffer): FileChunkBuilder {
    this.instance.chunkBytes = chunkBytes;
    return this;
  }

  build(): FileChunk {
    return this.instance;
  }
}

// A container is a logical grouping of blobs, similar to a directory in a file
// system.
export class Container implements Codec {
  id: string = "";

  static decodeNullable(decoder: Decoder): Container | null {
    if (decoder.isNextNil()) return null;
    return Container.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): Container {
    const o = new Container();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "id") {
        this.id = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("id");
    encoder.writeString(this.id);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): ContainerBuilder {
    return new ContainerBuilder();
  }
}

export class ContainerBuilder {
  instance: Container = new Container();

  withId(id: string): ContainerBuilder {
    this.instance.id = id;
    return this;
  }

  build(): Container {
    return this.instance;
  }
}

// A wrapper object around a list of containers.
export class ContainerList implements Codec {
  containers: Array<Container> = new Array<Container>();

  static decodeNullable(decoder: Decoder): ContainerList | null {
    if (decoder.isNextNil()) return null;
    return ContainerList.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ContainerList {
    const o = new ContainerList();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "containers") {
        this.containers = decoder.readArray(
          (decoder: Decoder): Container => {
            return Container.decode(decoder);
          }
        );
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("containers");
    encoder.writeArray(
      this.containers,
      (encoder: Writer, item: Container): void => {
        item.encode(encoder);
      }
    );
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): ContainerListBuilder {
    return new ContainerListBuilder();
  }
}

export class ContainerListBuilder {
  instance: ContainerList = new ContainerList();

  withContainers(containers: Array<Container>): ContainerListBuilder {
    this.instance.containers = containers;
    return this;
  }

  build(): ContainerList {
    return this.instance;
  }
}

// A blob is a representation of an object in a blobstore, similar to a file in a
// file system.
export class Blob implements Codec {
  id: string = "";
  container: Container = new Container();
  byteSize: u64 = 0;

  static decodeNullable(decoder: Decoder): Blob | null {
    if (decoder.isNextNil()) return null;
    return Blob.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): Blob {
    const o = new Blob();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "id") {
        this.id = decoder.readString();
      } else if (field == "container") {
        this.container = Container.decode(decoder);
      } else if (field == "byteSize") {
        this.byteSize = decoder.readUInt64();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(3);
    encoder.writeString("id");
    encoder.writeString(this.id);
    encoder.writeString("container");
    this.container.encode(encoder);
    encoder.writeString("byteSize");
    encoder.writeUInt64(this.byteSize);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): BlobBuilder {
    return new BlobBuilder();
  }
}

export class BlobBuilder {
  instance: Blob = new Blob();

  withId(id: string): BlobBuilder {
    this.instance.id = id;
    return this;
  }

  withContainer(container: Container): BlobBuilder {
    this.instance.container = container;
    return this;
  }

  withByteSize(byteSize: u64): BlobBuilder {
    this.instance.byteSize = byteSize;
    return this;
  }

  build(): Blob {
    return this.instance;
  }
}

// A wrapper object around a list of blobs.
export class BlobList implements Codec {
  blobs: Array<Blob> = new Array<Blob>();

  static decodeNullable(decoder: Decoder): BlobList | null {
    if (decoder.isNextNil()) return null;
    return BlobList.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): BlobList {
    const o = new BlobList();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "blobs") {
        this.blobs = decoder.readArray(
          (decoder: Decoder): Blob => {
            return Blob.decode(decoder);
          }
        );
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("blobs");
    encoder.writeArray(this.blobs, (encoder: Writer, item: Blob): void => {
      item.encode(encoder);
    });
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): BlobListBuilder {
    return new BlobListBuilder();
  }
}

export class BlobListBuilder {
  instance: BlobList = new BlobList();

  withBlobs(blobs: Array<Blob>): BlobListBuilder {
    this.instance.blobs = blobs;
    return this;
  }

  build(): BlobList {
    return this.instance;
  }
}

// Used to request a download from a blobstore
export class StreamRequest implements Codec {
  id: string = "";
  container: Container = new Container();
  chunkSize: u64 = 0;
  context: string | null = null;

  static decodeNullable(decoder: Decoder): StreamRequest | null {
    if (decoder.isNextNil()) return null;
    return StreamRequest.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): StreamRequest {
    const o = new StreamRequest();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "id") {
        this.id = decoder.readString();
      } else if (field == "container") {
        this.container = Container.decode(decoder);
      } else if (field == "chunkSize") {
        this.chunkSize = decoder.readUInt64();
      } else if (field == "context") {
        if (decoder.isNextNil()) {
          this.context = null;
        } else {
          this.context = decoder.readString();
        }
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(4);
    encoder.writeString("id");
    encoder.writeString(this.id);
    encoder.writeString("container");
    this.container.encode(encoder);
    encoder.writeString("chunkSize");
    encoder.writeUInt64(this.chunkSize);
    encoder.writeString("context");
    if (this.context === null) {
      encoder.writeNil();
    } else {
      encoder.writeString(this.context);
    }
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): StreamRequestBuilder {
    return new StreamRequestBuilder();
  }
}

export class StreamRequestBuilder {
  instance: StreamRequest = new StreamRequest();

  withId(id: string): StreamRequestBuilder {
    this.instance.id = id;
    return this;
  }

  withContainer(container: Container): StreamRequestBuilder {
    this.instance.container = container;
    return this;
  }

  withChunkSize(chunkSize: u64): StreamRequestBuilder {
    this.instance.chunkSize = chunkSize;
    return this;
  }

  withContext(context: string | null): StreamRequestBuilder {
    this.instance.context = context;
    return this;
  }

  build(): StreamRequest {
    return this.instance;
  }
}

export class Transfer implements Codec {
  blobId: string = "";
  container: Container = new Container();
  chunkSize: u64 = 0;
  totalSize: u64 = 0;
  totalChunks: u64 = 0;
  context: string | null = null;

  static decodeNullable(decoder: Decoder): Transfer | null {
    if (decoder.isNextNil()) return null;
    return Transfer.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): Transfer {
    const o = new Transfer();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "blobId") {
        this.blobId = decoder.readString();
      } else if (field == "container") {
        this.container = Container.decode(decoder);
      } else if (field == "chunkSize") {
        this.chunkSize = decoder.readUInt64();
      } else if (field == "totalSize") {
        this.totalSize = decoder.readUInt64();
      } else if (field == "totalChunks") {
        this.totalChunks = decoder.readUInt64();
      } else if (field == "context") {
        if (decoder.isNextNil()) {
          this.context = null;
        } else {
          this.context = decoder.readString();
        }
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(6);
    encoder.writeString("blobId");
    encoder.writeString(this.blobId);
    encoder.writeString("container");
    this.container.encode(encoder);
    encoder.writeString("chunkSize");
    encoder.writeUInt64(this.chunkSize);
    encoder.writeString("totalSize");
    encoder.writeUInt64(this.totalSize);
    encoder.writeString("totalChunks");
    encoder.writeUInt64(this.totalChunks);
    encoder.writeString("context");
    if (this.context === null) {
      encoder.writeNil();
    } else {
      encoder.writeString(this.context);
    }
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): TransferBuilder {
    return new TransferBuilder();
  }
}

export class TransferBuilder {
  instance: Transfer = new Transfer();

  withBlobId(blobId: string): TransferBuilder {
    this.instance.blobId = blobId;
    return this;
  }

  withContainer(container: Container): TransferBuilder {
    this.instance.container = container;
    return this;
  }

  withChunkSize(chunkSize: u64): TransferBuilder {
    this.instance.chunkSize = chunkSize;
    return this;
  }

  withTotalSize(totalSize: u64): TransferBuilder {
    this.instance.totalSize = totalSize;
    return this;
  }

  withTotalChunks(totalChunks: u64): TransferBuilder {
    this.instance.totalChunks = totalChunks;
    return this;
  }

  withContext(context: string | null): TransferBuilder {
    this.instance.context = context;
    return this;
  }

  build(): Transfer {
    return this.instance;
  }
}
