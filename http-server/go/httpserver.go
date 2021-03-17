package httpserver

import (
	msgpack "github.com/wapc/tinygo-msgpack"
	wapc "github.com/wapc/wapc-guest-tinygo"
)

type Handlers struct {
	HandleRequest func(request Request) (Response, error)
}

func (h Handlers) Register() {
	if h.HandleRequest != nil {
		HandleRequestHandler = h.HandleRequest
		wapc.RegisterFunction("HandleRequest", HandleRequestWrapper)
	}
}

var (
	HandleRequestHandler func(request Request) (Response, error)
)

func HandleRequestWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var request Request
	request.Decode(&decoder)
	response, err := HandleRequestHandler(request)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

type Request struct {
	Method      string
	Path        string
	QueryString string
	Header      map[string]string
	Body        []byte
}

func DecodeRequestNullable(decoder *msgpack.Decoder) (*Request, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeRequest(decoder)
	return &decoded, err
}

func DecodeRequest(decoder *msgpack.Decoder) (Request, error) {
	var o Request
	err := o.Decode(decoder)
	return o, err
}

func (o *Request) Decode(decoder *msgpack.Decoder) error {
	numFields, err := decoder.ReadMapSize()
	if err != nil {
		return err
	}

	for numFields > 0 {
		numFields--
		field, err := decoder.ReadString()
		if err != nil {
			return err
		}
		switch field {
		case "method":
			o.Method, err = decoder.ReadString()
		case "path":
			o.Path, err = decoder.ReadString()
		case "queryString":
			o.QueryString, err = decoder.ReadString()
		case "header":
			mapSize, err := decoder.ReadMapSize()
			if err != nil {
				return err
			}
			o.Header = make(map[string]string, mapSize)
			for mapSize > 0 {
				mapSize--
				key, err := decoder.ReadString()
				if err != nil {
					return err
				}
				value, err := decoder.ReadString()
				if err != nil {
					return err
				}
				o.Header[key] = value
			}
		case "body":
			o.Body, err = decoder.ReadByteArray()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *Request) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(5)
	encoder.WriteString("method")
	encoder.WriteString(o.Method)
	encoder.WriteString("path")
	encoder.WriteString(o.Path)
	encoder.WriteString("queryString")
	encoder.WriteString(o.QueryString)
	encoder.WriteString("header")
	encoder.WriteMapSize(uint32(len(o.Header)))
	if o.Header != nil { // TinyGo bug: ranging over nil maps panics.
		for k, v := range o.Header {
			encoder.WriteString(k)
			encoder.WriteString(v)
		}
	}
	encoder.WriteString("body")
	encoder.WriteByteArray(o.Body)

	return nil
}

type Response struct {
	StatusCode uint32
	Status     string
	Header     map[string]string
	Body       []byte
}

func DecodeResponseNullable(decoder *msgpack.Decoder) (*Response, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeResponse(decoder)
	return &decoded, err
}

func DecodeResponse(decoder *msgpack.Decoder) (Response, error) {
	var o Response
	err := o.Decode(decoder)
	return o, err
}

func (o *Response) Decode(decoder *msgpack.Decoder) error {
	numFields, err := decoder.ReadMapSize()
	if err != nil {
		return err
	}

	for numFields > 0 {
		numFields--
		field, err := decoder.ReadString()
		if err != nil {
			return err
		}
		switch field {
		case "statusCode":
			o.StatusCode, err = decoder.ReadUint32()
		case "status":
			o.Status, err = decoder.ReadString()
		case "header":
			mapSize, err := decoder.ReadMapSize()
			if err != nil {
				return err
			}
			o.Header = make(map[string]string, mapSize)
			for mapSize > 0 {
				mapSize--
				key, err := decoder.ReadString()
				if err != nil {
					return err
				}
				value, err := decoder.ReadString()
				if err != nil {
					return err
				}
				o.Header[key] = value
			}
		case "body":
			o.Body, err = decoder.ReadByteArray()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *Response) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(4)
	encoder.WriteString("statusCode")
	encoder.WriteUint32(o.StatusCode)
	encoder.WriteString("status")
	encoder.WriteString(o.Status)
	encoder.WriteString("header")
	encoder.WriteMapSize(uint32(len(o.Header)))
	if o.Header != nil { // TinyGo bug: ranging over nil maps panics.
		for k, v := range o.Header {
			encoder.WriteString(k)
			encoder.WriteString(v)
		}
	}
	encoder.WriteString("body")
	encoder.WriteByteArray(o.Body)

	return nil
}
