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

  HealthRequest(request: HealthCheckRequest): HealthCheckResponse {
    const payload = hostCall(
      this.binding,
      "core",
      "HealthRequest",
      request.toBuffer()
    );
    const decoder = new Decoder(payload);
    return HealthCheckResponse.decode(decoder);
  }
}

export class Handlers {
  static registerHealthRequest(
    handler: (request: HealthCheckRequest) => HealthCheckResponse
  ): void {
    HealthRequestHandler = handler;
    register("HealthRequest", HealthRequestWrapper);
  }
}

var HealthRequestHandler: (request: HealthCheckRequest) => HealthCheckResponse;
function HealthRequestWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const request = new HealthCheckRequest();
  request.decode(decoder);
  const response = HealthRequestHandler(request);
  return response.toBuffer();
}

export class CapabilityConfiguration implements Codec {
  module: string = "";
  values: Map<string, string> = new Map<string, string>();

  static decodeNullable(decoder: Decoder): CapabilityConfiguration | null {
    if (decoder.isNextNil()) return null;
    return CapabilityConfiguration.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): CapabilityConfiguration {
    const o = new CapabilityConfiguration();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "module") {
        this.module = decoder.readString();
      } else if (field == "values") {
        this.values = decoder.readMap(
          (decoder: Decoder): string => {
            return decoder.readString();
          },
          (decoder: Decoder): string => {
            return decoder.readString();
          }
        );
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("module");
    encoder.writeString(this.module);
    encoder.writeString("values");
    encoder.writeMap(
      this.values,
      (encoder: Writer, key: string): void => {
        encoder.writeString(key);
      },
      (encoder: Writer, value: string): void => {
        encoder.writeString(value);
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

  static newBuilder(): CapabilityConfigurationBuilder {
    return new CapabilityConfigurationBuilder();
  }
}

export class CapabilityConfigurationBuilder {
  instance: CapabilityConfiguration = new CapabilityConfiguration();

  withModule(module: string): CapabilityConfigurationBuilder {
    this.instance.module = module;
    return this;
  }

  withValues(values: Map<string, string>): CapabilityConfigurationBuilder {
    this.instance.values = values;
    return this;
  }

  build(): CapabilityConfiguration {
    return this.instance;
  }
}

export class HealthCheckRequest implements Codec {
  placeholder: bool = false;

  static decodeNullable(decoder: Decoder): HealthCheckRequest | null {
    if (decoder.isNextNil()) return null;
    return HealthCheckRequest.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): HealthCheckRequest {
    const o = new HealthCheckRequest();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "placeholder") {
        this.placeholder = decoder.readBool();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(1);
    encoder.writeString("placeholder");
    encoder.writeBool(this.placeholder);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): HealthCheckRequestBuilder {
    return new HealthCheckRequestBuilder();
  }
}

export class HealthCheckRequestBuilder {
  instance: HealthCheckRequest = new HealthCheckRequest();

  withPlaceholder(placeholder: bool): HealthCheckRequestBuilder {
    this.instance.placeholder = placeholder;
    return this;
  }

  build(): HealthCheckRequest {
    return this.instance;
  }
}

export class HealthCheckResponse implements Codec {
  healthy: bool = false;
  message: string = "";

  static decodeNullable(decoder: Decoder): HealthCheckResponse | null {
    if (decoder.isNextNil()) return null;
    return HealthCheckResponse.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): HealthCheckResponse {
    const o = new HealthCheckResponse();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "healthy") {
        this.healthy = decoder.readBool();
      } else if (field == "message") {
        this.message = decoder.readString();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(2);
    encoder.writeString("healthy");
    encoder.writeBool(this.healthy);
    encoder.writeString("message");
    encoder.writeString(this.message);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): HealthCheckResponseBuilder {
    return new HealthCheckResponseBuilder();
  }
}

export class HealthCheckResponseBuilder {
  instance: HealthCheckResponse = new HealthCheckResponse();

  withHealthy(healthy: bool): HealthCheckResponseBuilder {
    this.instance.healthy = healthy;
    return this;
  }

  withMessage(message: string): HealthCheckResponseBuilder {
    this.instance.message = message;
    return this;
  }

  build(): HealthCheckResponse {
    return this.instance;
  }
}
