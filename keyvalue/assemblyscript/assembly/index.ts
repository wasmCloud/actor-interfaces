import { Decoder, Writer, Encoder, Sizer, Codec } from "@wapc/as-msgpack";

import { hostCall } from "@wapc/as-guest";
export class Host {
  binding: string;

  constructor(binding: string = "default") {
    this.binding = binding;
  }

  get(key: string): GetResponse {
    const inputArgs = new GetArgs();
    inputArgs.key = key;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Get",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return GetResponse.decode(decoder);
  }

  add(key: string, value: i32): AddResponse {
    const inputArgs = new AddArgs();
    inputArgs.key = key;
    inputArgs.value = value;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Add",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return AddResponse.decode(decoder);
  }

  set(key: string, value: string, expires: i32): SetResponse {
    const inputArgs = new SetArgs();
    inputArgs.key = key;
    inputArgs.value = value;
    inputArgs.expires = expires;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Set",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return SetResponse.decode(decoder);
  }

  del(key: string): DelResponse {
    const inputArgs = new DelArgs();
    inputArgs.key = key;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Del",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return DelResponse.decode(decoder);
  }

  clear(key: string): DelResponse {
    const inputArgs = new ClearArgs();
    inputArgs.key = key;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Clear",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return DelResponse.decode(decoder);
  }

  range(key: string, start: i32, stop: i32): ListRangeResponse {
    const inputArgs = new RangeArgs();
    inputArgs.key = key;
    inputArgs.start = start;
    inputArgs.stop = stop;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Range",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return ListRangeResponse.decode(decoder);
  }

  push(key: string, value: string): ListResponse {
    const inputArgs = new PushArgs();
    inputArgs.key = key;
    inputArgs.value = value;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "Push",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return ListResponse.decode(decoder);
  }

  listItemDelete(key: string, value: string): ListResponse {
    const inputArgs = new ListItemDeleteArgs();
    inputArgs.key = key;
    inputArgs.value = value;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "ListItemDelete",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return ListResponse.decode(decoder);
  }

  setAdd(key: string, value: string): SetOperationResponse {
    const inputArgs = new SetAddArgs();
    inputArgs.key = key;
    inputArgs.value = value;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "SetAdd",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return SetOperationResponse.decode(decoder);
  }

  setRemove(key: string, value: string): SetOperationResponse {
    const inputArgs = new SetRemoveArgs();
    inputArgs.key = key;
    inputArgs.value = value;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "SetRemove",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return SetOperationResponse.decode(decoder);
  }

  setUnion(keys: Array<string>): SetQueryResponse {
    const inputArgs = new SetUnionArgs();
    inputArgs.keys = keys;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "SetUnion",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return SetQueryResponse.decode(decoder);
  }

  setIntersection(keys: Array<string>): SetQueryResponse {
    const inputArgs = new SetIntersectionArgs();
    inputArgs.keys = keys;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "SetIntersection",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return SetQueryResponse.decode(decoder);
  }

  setQuery(key: string): SetQueryResponse {
    const inputArgs = new SetQueryArgs();
    inputArgs.key = key;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "SetQuery",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return SetQueryResponse.decode(decoder);
  }

  keyExists(key: string): GetResponse {
    const inputArgs = new KeyExistsArgs();
    inputArgs.key = key;
    const payload = hostCall(
      this.binding,
      "wasmcloud:keyvalue",
      "KeyExists",
      inputArgs.toBuffer()
    );
    const decoder = new Decoder(payload);
    return GetResponse.decode(decoder);
  }
}

export class GetArgs implements Codec {
  key: string = "";

  static decodeNullable(decoder: Decoder): GetArgs | null {
    if (decoder.isNextNil()) return null;
    return GetArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): GetArgs {
    const o = new GetArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("key");
    encoder.writeString(this.key);
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

export class AddArgs implements Codec {
  key: string = "";
  value: i32 = 0;

  static decodeNullable(decoder: Decoder): AddArgs | null {
    if (decoder.isNextNil()) return null;
    return AddArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): AddArgs {
    const o = new AddArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "value") {
        this.value = decoder.readInt32();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("value");
    encoder.writeInt32(this.value);
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

export class SetArgs implements Codec {
  key: string = "";
  value: string = "";
  expires: i32 = 0;

  static decodeNullable(decoder: Decoder): SetArgs | null {
    if (decoder.isNextNil()) return null;
    return SetArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetArgs {
    const o = new SetArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "value") {
        this.value = decoder.readString();
      } else if (field == "expires") {
        this.expires = decoder.readInt32();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(3);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("value");
    encoder.writeString(this.value);
    encoder.writeString("expires");
    encoder.writeInt32(this.expires);
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

export class DelArgs implements Codec {
  key: string = "";

  static decodeNullable(decoder: Decoder): DelArgs | null {
    if (decoder.isNextNil()) return null;
    return DelArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): DelArgs {
    const o = new DelArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("key");
    encoder.writeString(this.key);
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

export class ClearArgs implements Codec {
  key: string = "";

  static decodeNullable(decoder: Decoder): ClearArgs | null {
    if (decoder.isNextNil()) return null;
    return ClearArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ClearArgs {
    const o = new ClearArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("key");
    encoder.writeString(this.key);
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

export class RangeArgs implements Codec {
  key: string = "";
  start: i32 = 0;
  stop: i32 = 0;

  static decodeNullable(decoder: Decoder): RangeArgs | null {
    if (decoder.isNextNil()) return null;
    return RangeArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): RangeArgs {
    const o = new RangeArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "start") {
        this.start = decoder.readInt32();
      } else if (field == "stop") {
        this.stop = decoder.readInt32();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(3);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("start");
    encoder.writeInt32(this.start);
    encoder.writeString("stop");
    encoder.writeInt32(this.stop);
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

export class PushArgs implements Codec {
  key: string = "";
  value: string = "";

  static decodeNullable(decoder: Decoder): PushArgs | null {
    if (decoder.isNextNil()) return null;
    return PushArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): PushArgs {
    const o = new PushArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "value") {
        this.value = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("value");
    encoder.writeString(this.value);
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

export class ListItemDeleteArgs implements Codec {
  key: string = "";
  value: string = "";

  static decodeNullable(decoder: Decoder): ListItemDeleteArgs | null {
    if (decoder.isNextNil()) return null;
    return ListItemDeleteArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ListItemDeleteArgs {
    const o = new ListItemDeleteArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "value") {
        this.value = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("value");
    encoder.writeString(this.value);
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

export class SetAddArgs implements Codec {
  key: string = "";
  value: string = "";

  static decodeNullable(decoder: Decoder): SetAddArgs | null {
    if (decoder.isNextNil()) return null;
    return SetAddArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetAddArgs {
    const o = new SetAddArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "value") {
        this.value = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("value");
    encoder.writeString(this.value);
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

export class SetRemoveArgs implements Codec {
  key: string = "";
  value: string = "";

  static decodeNullable(decoder: Decoder): SetRemoveArgs | null {
    if (decoder.isNextNil()) return null;
    return SetRemoveArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetRemoveArgs {
    const o = new SetRemoveArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else if (field == "value") {
        this.value = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("key");
    encoder.writeString(this.key);
    encoder.writeString("value");
    encoder.writeString(this.value);
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

export class SetUnionArgs implements Codec {
  keys: Array<string> = new Array<string>();

  static decodeNullable(decoder: Decoder): SetUnionArgs | null {
    if (decoder.isNextNil()) return null;
    return SetUnionArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetUnionArgs {
    const o = new SetUnionArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "keys") {
        this.keys = decoder.readArray((decoder: Decoder): string => {
          return decoder.readString();
        });
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("keys");
    encoder.writeArray(this.keys, (encoder: Writer, item: string): void => {
      encoder.writeString(item);
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
}

export class SetIntersectionArgs implements Codec {
  keys: Array<string> = new Array<string>();

  static decodeNullable(decoder: Decoder): SetIntersectionArgs | null {
    if (decoder.isNextNil()) return null;
    return SetIntersectionArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetIntersectionArgs {
    const o = new SetIntersectionArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "keys") {
        this.keys = decoder.readArray((decoder: Decoder): string => {
          return decoder.readString();
        });
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("keys");
    encoder.writeArray(this.keys, (encoder: Writer, item: string): void => {
      encoder.writeString(item);
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
}

export class SetQueryArgs implements Codec {
  key: string = "";

  static decodeNullable(decoder: Decoder): SetQueryArgs | null {
    if (decoder.isNextNil()) return null;
    return SetQueryArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetQueryArgs {
    const o = new SetQueryArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("key");
    encoder.writeString(this.key);
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

export class KeyExistsArgs implements Codec {
  key: string = "";

  static decodeNullable(decoder: Decoder): KeyExistsArgs | null {
    if (decoder.isNextNil()) return null;
    return KeyExistsArgs.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): KeyExistsArgs {
    const o = new KeyExistsArgs();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("key");
    encoder.writeString(this.key);
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

export class GetResponse implements Codec {
  value: string = "";
  exists: bool = false;

  static decodeNullable(decoder: Decoder): GetResponse | null {
    if (decoder.isNextNil()) return null;
    return GetResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): GetResponse {
    const o = new GetResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "value") {
        this.value = decoder.readString();
      } else if (field == "exists") {
        this.exists = decoder.readBool();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("value");
    encoder.writeString(this.value);
    encoder.writeString("exists");
    encoder.writeBool(this.exists);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): GetResponseBuilder {
    return new GetResponseBuilder();
  }
}

export class GetResponseBuilder {
  instance: GetResponse = new GetResponse();

  withValue(value: string): GetResponseBuilder {
    this.instance.value = value;
    return this;
  }

  withExists(exists: bool): GetResponseBuilder {
    this.instance.exists = exists;
    return this;
  }

  build(): GetResponse {
    return this.instance;
  }
}

export class AddResponse implements Codec {
  value: i32 = 0;

  static decodeNullable(decoder: Decoder): AddResponse | null {
    if (decoder.isNextNil()) return null;
    return AddResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): AddResponse {
    const o = new AddResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "value") {
        this.value = decoder.readInt32();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("value");
    encoder.writeInt32(this.value);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): AddResponseBuilder {
    return new AddResponseBuilder();
  }
}

export class AddResponseBuilder {
  instance: AddResponse = new AddResponse();

  withValue(value: i32): AddResponseBuilder {
    this.instance.value = value;
    return this;
  }

  build(): AddResponse {
    return this.instance;
  }
}

export class DelResponse implements Codec {
  key: string = "";

  static decodeNullable(decoder: Decoder): DelResponse | null {
    if (decoder.isNextNil()) return null;
    return DelResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): DelResponse {
    const o = new DelResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "key") {
        this.key = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("key");
    encoder.writeString(this.key);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): DelResponseBuilder {
    return new DelResponseBuilder();
  }
}

export class DelResponseBuilder {
  instance: DelResponse = new DelResponse();

  withKey(key: string): DelResponseBuilder {
    this.instance.key = key;
    return this;
  }

  build(): DelResponse {
    return this.instance;
  }
}

export class ListRangeResponse implements Codec {
  values: Array<string> = new Array<string>();

  static decodeNullable(decoder: Decoder): ListRangeResponse | null {
    if (decoder.isNextNil()) return null;
    return ListRangeResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ListRangeResponse {
    const o = new ListRangeResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "values") {
        this.values = decoder.readArray((decoder: Decoder): string => {
          return decoder.readString();
        });
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("values");
    encoder.writeArray(this.values, (encoder: Writer, item: string): void => {
      encoder.writeString(item);
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

  static newBuilder(): ListRangeResponseBuilder {
    return new ListRangeResponseBuilder();
  }
}

export class ListRangeResponseBuilder {
  instance: ListRangeResponse = new ListRangeResponse();

  withValues(values: Array<string>): ListRangeResponseBuilder {
    this.instance.values = values;
    return this;
  }

  build(): ListRangeResponse {
    return this.instance;
  }
}

export class ListResponse implements Codec {
  newCount: i32 = 0;

  static decodeNullable(decoder: Decoder): ListResponse | null {
    if (decoder.isNextNil()) return null;
    return ListResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): ListResponse {
    const o = new ListResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "newCount") {
        this.newCount = decoder.readInt32();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("newCount");
    encoder.writeInt32(this.newCount);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): ListResponseBuilder {
    return new ListResponseBuilder();
  }
}

export class ListResponseBuilder {
  instance: ListResponse = new ListResponse();

  withNewCount(newCount: i32): ListResponseBuilder {
    this.instance.newCount = newCount;
    return this;
  }

  build(): ListResponse {
    return this.instance;
  }
}

export class SetResponse implements Codec {
  value: string = "";

  static decodeNullable(decoder: Decoder): SetResponse | null {
    if (decoder.isNextNil()) return null;
    return SetResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetResponse {
    const o = new SetResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "value") {
        this.value = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("value");
    encoder.writeString(this.value);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): SetResponseBuilder {
    return new SetResponseBuilder();
  }
}

export class SetResponseBuilder {
  instance: SetResponse = new SetResponse();

  withValue(value: string): SetResponseBuilder {
    this.instance.value = value;
    return this;
  }

  build(): SetResponse {
    return this.instance;
  }
}

export class SetOperationResponse implements Codec {
  new_count: i32 = 0;

  static decodeNullable(decoder: Decoder): SetOperationResponse | null {
    if (decoder.isNextNil()) return null;
    return SetOperationResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetOperationResponse {
    const o = new SetOperationResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "new_count") {
        this.new_count = decoder.readInt32();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("new_count");
    encoder.writeInt32(this.new_count);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): SetOperationResponseBuilder {
    return new SetOperationResponseBuilder();
  }
}

export class SetOperationResponseBuilder {
  instance: SetOperationResponse = new SetOperationResponse();

  withNew_count(new_count: i32): SetOperationResponseBuilder {
    this.instance.new_count = new_count;
    return this;
  }

  build(): SetOperationResponse {
    return this.instance;
  }
}

export class SetQueryResponse implements Codec {
  values: Array<string> = new Array<string>();

  static decodeNullable(decoder: Decoder): SetQueryResponse | null {
    if (decoder.isNextNil()) return null;
    return SetQueryResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): SetQueryResponse {
    const o = new SetQueryResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "values") {
        this.values = decoder.readArray((decoder: Decoder): string => {
          return decoder.readString();
        });
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("values");
    encoder.writeArray(this.values, (encoder: Writer, item: string): void => {
      encoder.writeString(item);
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

  static newBuilder(): SetQueryResponseBuilder {
    return new SetQueryResponseBuilder();
  }
}

export class SetQueryResponseBuilder {
  instance: SetQueryResponse = new SetQueryResponse();

  withValues(values: Array<string>): SetQueryResponseBuilder {
    this.instance.values = values;
    return this;
  }

  build(): SetQueryResponse {
    return this.instance;
  }
}
