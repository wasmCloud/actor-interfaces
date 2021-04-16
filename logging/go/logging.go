package logging

import (
	msgpack "github.com/wapc/tinygo-msgpack"
	wapc "github.com/wapc/wapc-guest-tinygo"
)

type Host struct {
	binding string
}

func NewHost(binding string) *Host {
	return &Host{
		binding: binding,
	}
}

// Writes a log message to specified target and level. `target` is used to filter
// logs to a specific target, e.g.  for logging to your specific actor name, and
// can be left as an empty string. `level` can be one of the following log levels:
// `error`, `warn`, `info`, `debug`, `trace`. It is up to the provider to ensure
// the `level` value is valid.
func (h *Host) WriteLog(target string, level string, text string) error {
	inputArgs := WriteLogArgs{
		Target: target,
		Level:  level,
		Text:   text,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return err
	}
	_, err = wapc.HostCall(
		h.binding,
		"wasmcloud:logging",
		"WriteLog",
		inputBytes,
	)
	return err
}

type WriteLogArgs struct {
	Target string
	Level  string
	Text   string
}

func DecodeWriteLogArgsNullable(decoder *msgpack.Decoder) (*WriteLogArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeWriteLogArgs(decoder)
	return &decoded, err
}

func DecodeWriteLogArgs(decoder *msgpack.Decoder) (WriteLogArgs, error) {
	var o WriteLogArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *WriteLogArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "target":
			o.Target, err = decoder.ReadString()
		case "level":
			o.Level, err = decoder.ReadString()
		case "text":
			o.Text, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *WriteLogArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(3)
	encoder.WriteString("target")
	encoder.WriteString(o.Target)
	encoder.WriteString("level")
	encoder.WriteString(o.Level)
	encoder.WriteString("text")
	encoder.WriteString(o.Text)

	return nil
}
