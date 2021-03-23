import { register, hostCall } from "@wapc/as-guest";
import {
  Decoder,
  Writer,
  Encoder,
  Sizer,
  Codec,
  Value,
} from "@wapc/as-msgpack";

export class Host {
  binding: string;

  constructor(binding: string) {
    this.binding = binding;
  }

  CreateContainer(id: string): Container {
    const inputArgs = new CreateContainerArgs();
    inputArgs.id = id;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "CreateContainer",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return Container.decode(decoder);
  }

  RemoveContainer(id: string): BlobstoreResult {
    const inputArgs = new RemoveContainerArgs();
    inputArgs.id = id;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "RemoveContainer",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return BlobstoreResult.decode(decoder);
  }

  RemoveObject(id: string, container_id: string): BlobstoreResult {
    const inputArgs = new RemoveObjectArgs();
    inputArgs.id = id;
    inputArgs.container_id = container_id;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "RemoveObject",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return BlobstoreResult.decode(decoder);
  }

  ListObjects(container_id: string): BlobList {
    const inputArgs = new ListObjectsArgs();
    inputArgs.container_id = container_id;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "ListObjects",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return BlobList.decode(decoder);
  }

  UploadChunk(chunk: FileChunk): void {
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "UploadChunk",
      chunk.toBuffer()
    );
  }

  StartDownload(
    blob_id: string,
    container_id: string,
    chunk_size: u64,
    context: Value<string> | null
  ): BlobstoreResult {
    const inputArgs = new StartDownloadArgs();
    inputArgs.blob_id = blob_id;
    inputArgs.container_id = container_id;
    inputArgs.chunk_size = chunk_size;
    inputArgs.context = context;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "StartDownload",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return BlobstoreResult.decode(decoder);
  }

  StartUpload(blob: FileChunk): BlobstoreResult {
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "StartUpload",
      blob.toBuffer()
    );
    const decoder = new Decoder(payload);
    return BlobstoreResult.decode(decoder);
  }

  GetObjectInfo(blob_id: string, container_id: string): Blob {
    const inputArgs = new GetObjectInfoArgs();
    inputArgs.blob_id = blob_id;
    inputArgs.container_id = container_id;
    const payload = hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "GetObjectInfo",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return Blob.decode(decoder);
  }

  ReceiveChunk(chunk: FileChunk): void {
    hostCall(
      this.binding,
      "wasmcloud:blobstore",
      "ReceiveChunk",
      chunk.toBuffer()
    );
  }
}

export class Handlers {
  static registerCreateContainer(handler: (id: string) => Container): void {
    CreateContainerHandler = handler;
    register("CreateContainer", CreateContainerWrapper);
  }

  static registerRemoveContainer(
    handler: (id: string) => BlobstoreResult
  ): void {
    RemoveContainerHandler = handler;
    register("RemoveContainer", RemoveContainerWrapper);
  }

  static registerRemoveObject(
    handler: (id: string, container_id: string) => BlobstoreResult
  ): void {
    RemoveObjectHandler = handler;
    register("RemoveObject", RemoveObjectWrapper);
  }

  static registerListObjects(
    handler: (container_id: string) => BlobList
  ): void {
    ListObjectsHandler = handler;
    register("ListObjects", ListObjectsWrapper);
  }

  static registerUploadChunk(handler: (chunk: FileChunk) => void): void {
    UploadChunkHandler = handler;
    register("UploadChunk", UploadChunkWrapper);
  }

  static registerStartDownload(
    handler: (
      blob_id: string,
      container_id: string,
      chunk_size: u64,
      context: Value<string> | null
    ) => BlobstoreResult
  ): void {
    StartDownloadHandler = handler;
    register("StartDownload", StartDownloadWrapper);
  }

  static registerStartUpload(
    handler: (blob: FileChunk) => BlobstoreResult
  ): void {
    StartUploadHandler = handler;
    register("StartUpload", StartUploadWrapper);
  }

  static registerGetObjectInfo(
    handler: (blob_id: string, container_id: string) => Blob
  ): void {
    GetObjectInfoHandler = handler;
    register("GetObjectInfo", GetObjectInfoWrapper);
  }

  static registerReceiveChunk(handler: (chunk: FileChunk) => void): void {
    ReceiveChunkHandler = handler;
    register("ReceiveChunk", ReceiveChunkWrapper);
  }
}

var CreateContainerHandler: (id: string) => Container;
function CreateContainerWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new CreateContainerArgs();
  inputArgs.decode(decoder);
  const response = CreateContainerHandler(inputArgs.id);
  return response.toBuffer();
}

var RemoveContainerHandler: (id: string) => BlobstoreResult;
function RemoveContainerWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new RemoveContainerArgs();
  inputArgs.decode(decoder);
  const response = RemoveContainerHandler(inputArgs.id);
  return response.toBuffer();
}

var RemoveObjectHandler: (id: string, container_id: string) => BlobstoreResult;
function RemoveObjectWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new RemoveObjectArgs();
  inputArgs.decode(decoder);
  const response = RemoveObjectHandler(inputArgs.id, inputArgs.container_id);
  return response.toBuffer();
}

var ListObjectsHandler: (container_id: string) => BlobList;
function ListObjectsWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new ListObjectsArgs();
  inputArgs.decode(decoder);
  const response = ListObjectsHandler(inputArgs.container_id);
  return response.toBuffer();
}

var UploadChunkHandler: (chunk: FileChunk) => void;
function UploadChunkWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const request = new FileChunk();
  request.decode(decoder);
  UploadChunkHandler(request);
  return new ArrayBuffer(0);
}

var StartDownloadHandler: (
  blob_id: string,
  container_id: string,
  chunk_size: u64,
  context: Value<string> | null
) => BlobstoreResult;
function StartDownloadWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new StartDownloadArgs();
  inputArgs.decode(decoder);
  const response = StartDownloadHandler(
    inputArgs.blob_id,
    inputArgs.container_id,
    inputArgs.chunk_size,
    inputArgs.context
  );
  return response.toBuffer();
}

var StartUploadHandler: (blob: FileChunk) => BlobstoreResult;
function StartUploadWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const request = new FileChunk();
  request.decode(decoder);
  const response = StartUploadHandler(request);
  return response.toBuffer();
}

var GetObjectInfoHandler: (blob_id: string, container_id: string) => Blob;
function GetObjectInfoWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const inputArgs = new GetObjectInfoArgs();
  inputArgs.decode(decoder);
  const response = GetObjectInfoHandler(
    inputArgs.blob_id,
    inputArgs.container_id
  );
  return response.toBuffer();
}

var ReceiveChunkHandler: (chunk: FileChunk) => void;
function ReceiveChunkWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const request = new FileChunk();
  request.decode(decoder);
  ReceiveChunkHandler(request);
  return new ArrayBuffer(0);
}

export class CreateContainerArgs implements Codec {
  id: string = "";

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
}

export class RemoveContainerArgs implements Codec {
  id: string = "";

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
}

export class RemoveObjectArgs implements Codec {
  id: string = "";
  container_id: string = "";

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

      if (field == "id") {
        this.id = decoder.readString();
      } else if (field == "container_id") {
        this.container_id = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("id");
    encoder.writeString(this.id);
    encoder.writeString("container_id");
    encoder.writeString(this.container_id);
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
  container_id: string = "";

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

      if (field == "container_id") {
        this.container_id = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("container_id");
    encoder.writeString(this.container_id);
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
  blob_id: string = "";
  container_id: string = "";
  chunk_size: u64 = 0;
  context: Value<string> | null = null;

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

      if (field == "blob_id") {
        this.blob_id = decoder.readString();
      } else if (field == "container_id") {
        this.container_id = decoder.readString();
      } else if (field == "chunk_size") {
        this.chunk_size = decoder.readUInt64();
      } else if (field == "context") {
        if (decoder.isNextNil()) {
          this.context = null;
        } else {
          this.context = new Value(decoder.readString());
        }
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(4);
    encoder.writeString("blob_id");
    encoder.writeString(this.blob_id);
    encoder.writeString("container_id");
    encoder.writeString(this.container_id);
    encoder.writeString("chunk_size");
    encoder.writeUInt64(this.chunk_size);
    encoder.writeString("context");
    if (this.context === null) {
      encoder.writeNil();
    } else {
      const unboxed = this.context!;
      encoder.writeString(unboxed.value);
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
}

export class GetObjectInfoArgs implements Codec {
  blob_id: string = "";
  container_id: string = "";

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

      if (field == "blob_id") {
        this.blob_id = decoder.readString();
      } else if (field == "container_id") {
        this.container_id = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("blob_id");
    encoder.writeString(this.blob_id);
    encoder.writeString("container_id");
    encoder.writeString(this.container_id);
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

export class BlobstoreResult implements Codec {
  success: bool = false;
  error: Value<string> | null = null;

  static decodeNullable(decoder: Decoder): BlobstoreResult | null {
    if (decoder.isNextNil()) return null;
    return BlobstoreResult.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): BlobstoreResult {
    const o = new BlobstoreResult();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "success") {
        this.success = decoder.readBool();
      } else if (field == "error") {
        if (decoder.isNextNil()) {
          this.error = null;
        } else {
          this.error = new Value(decoder.readString());
        }
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("success");
    encoder.writeBool(this.success);
    encoder.writeString("error");
    if (this.error === null) {
      encoder.writeNil();
    } else {
      const unboxed = this.error!;
      encoder.writeString(unboxed.value);
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

  static newBuilder(): BlobstoreResultBuilder {
    return new BlobstoreResultBuilder();
  }
}

export class BlobstoreResultBuilder {
  instance: BlobstoreResult = new BlobstoreResult();

  withSuccess(success: bool): BlobstoreResultBuilder {
    this.instance.success = success;
    return this;
  }

  withError(error: Value<string> | null): BlobstoreResultBuilder {
    this.instance.error = error;
    return this;
  }

  build(): BlobstoreResult {
    return this.instance;
  }
}

export class FileChunk implements Codec {
  sequenceNo: u64 = 0;
  container: Container = new Container();
  id: string = "";
  totalBytes: u64 = 0;
  chunkSize: u64 = 0;
  context: Value<string> | null = null;
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
          this.context = new Value(decoder.readString());
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
      const unboxed = this.context!;
      encoder.writeString(unboxed.value);
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

  withContext(context: Value<string> | null): FileChunkBuilder {
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

export class StreamRequest implements Codec {
  id: string = "";
  container: Container = new Container();
  chunkSize: u64 = 0;
  context: Value<string> | null = null;

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
          this.context = new Value(decoder.readString());
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
      const unboxed = this.context!;
      encoder.writeString(unboxed.value);
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

  withContext(context: Value<string> | null): StreamRequestBuilder {
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
  context: Value<string> | null = null;

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
          this.context = new Value(decoder.readString());
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
      const unboxed = this.context!;
      encoder.writeString(unboxed.value);
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

  withContext(context: Value<string> | null): TransferBuilder {
    this.instance.context = context;
    return this;
  }

  build(): Transfer {
    return this.instance;
  }
}
