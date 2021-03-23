package blobstore 

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

func (h *Host) CreateContainer(id string) (Container, error) {
	inputArgs := CreateContainerArgs{
		ID: id,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return Container{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"CreateContainer",
		inputBytes,
	)
	if err != nil {
		return Container{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeContainer(&decoder)
}

func (h *Host) RemoveContainer(id string) (BlobstoreResult, error) {
	inputArgs := RemoveContainerArgs{
		ID: id,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return BlobstoreResult{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"RemoveContainer",
		inputBytes,
	)
	if err != nil {
		return BlobstoreResult{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobstoreResult(&decoder)
}

func (h *Host) RemoveObject(id string, container_id string) (BlobstoreResult, error) {
	inputArgs := RemoveObjectArgs{
		ID:           id,
		Container_id: container_id,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return BlobstoreResult{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"RemoveObject",
		inputBytes,
	)
	if err != nil {
		return BlobstoreResult{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobstoreResult(&decoder)
}

func (h *Host) ListObjects(container_id string) (BlobList, error) {
	inputArgs := ListObjectsArgs{
		Container_id: container_id,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return BlobList{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"ListObjects",
		inputBytes,
	)
	if err != nil {
		return BlobList{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobList(&decoder)
}

func (h *Host) UploadChunk(chunk FileChunk) error {
	inputBytes, err := msgpack.ToBytes(&chunk)
	if err != nil {
		return err
	}
	_, err = wapc.HostCall(h.binding, "wasmcloud:blobstore", "UploadChunk", inputBytes)
	return err
}

func (h *Host) StartDownload(blob_id string, container_id string, chunk_size uint64, context *string) (BlobstoreResult, error) {
	inputArgs := StartDownloadArgs{
		Blob_id:      blob_id,
		Container_id: container_id,
		Chunk_size:   chunk_size,
		Context:      context,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return BlobstoreResult{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"StartDownload",
		inputBytes,
	)
	if err != nil {
		return BlobstoreResult{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobstoreResult(&decoder)
}

func (h *Host) StartUpload(blob FileChunk) (BlobstoreResult, error) {
	inputBytes, err := msgpack.ToBytes(&blob)
	if err != nil {
		return BlobstoreResult{}, err
	}
	payload, err := wapc.HostCall(h.binding, "wasmcloud:blobstore", "StartUpload", inputBytes)
	if err != nil {
		return BlobstoreResult{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobstoreResult(&decoder)
}

func (h *Host) GetObjectInfo(blob_id string, container_id string) (Blob, error) {
	inputArgs := GetObjectInfoArgs{
		Blob_id:      blob_id,
		Container_id: container_id,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return Blob{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"GetObjectInfo",
		inputBytes,
	)
	if err != nil {
		return Blob{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlob(&decoder)
}

func (h *Host) ReceiveChunk(chunk FileChunk) error {
	inputBytes, err := msgpack.ToBytes(&chunk)
	if err != nil {
		return err
	}
	_, err = wapc.HostCall(h.binding, "wasmcloud:blobstore", "ReceiveChunk", inputBytes)
	return err
}

type Handlers struct {
	CreateContainer func(id string) (Container, error)
	RemoveContainer func(id string) (BlobstoreResult, error)
	RemoveObject    func(id string, container_id string) (BlobstoreResult, error)
	ListObjects     func(container_id string) (BlobList, error)
	UploadChunk     func(chunk FileChunk) error
	StartDownload   func(blob_id string, container_id string, chunk_size uint64, context *string) (BlobstoreResult, error)
	StartUpload     func(blob FileChunk) (BlobstoreResult, error)
	GetObjectInfo   func(blob_id string, container_id string) (Blob, error)
	ReceiveChunk    func(chunk FileChunk) error
}

func (h Handlers) Register() {
	if h.CreateContainer != nil {
		CreateContainerHandler = h.CreateContainer
		wapc.RegisterFunction("CreateContainer", CreateContainerWrapper)
	}
	if h.RemoveContainer != nil {
		RemoveContainerHandler = h.RemoveContainer
		wapc.RegisterFunction("RemoveContainer", RemoveContainerWrapper)
	}
	if h.RemoveObject != nil {
		RemoveObjectHandler = h.RemoveObject
		wapc.RegisterFunction("RemoveObject", RemoveObjectWrapper)
	}
	if h.ListObjects != nil {
		ListObjectsHandler = h.ListObjects
		wapc.RegisterFunction("ListObjects", ListObjectsWrapper)
	}
	if h.UploadChunk != nil {
		UploadChunkHandler = h.UploadChunk
		wapc.RegisterFunction("UploadChunk", UploadChunkWrapper)
	}
	if h.StartDownload != nil {
		StartDownloadHandler = h.StartDownload
		wapc.RegisterFunction("StartDownload", StartDownloadWrapper)
	}
	if h.StartUpload != nil {
		StartUploadHandler = h.StartUpload
		wapc.RegisterFunction("StartUpload", StartUploadWrapper)
	}
	if h.GetObjectInfo != nil {
		GetObjectInfoHandler = h.GetObjectInfo
		wapc.RegisterFunction("GetObjectInfo", GetObjectInfoWrapper)
	}
	if h.ReceiveChunk != nil {
		ReceiveChunkHandler = h.ReceiveChunk
		wapc.RegisterFunction("ReceiveChunk", ReceiveChunkWrapper)
	}
}

var (
	CreateContainerHandler func(id string) (Container, error)
	RemoveContainerHandler func(id string) (BlobstoreResult, error)
	RemoveObjectHandler    func(id string, container_id string) (BlobstoreResult, error)
	ListObjectsHandler     func(container_id string) (BlobList, error)
	UploadChunkHandler     func(chunk FileChunk) error
	StartDownloadHandler   func(blob_id string, container_id string, chunk_size uint64, context *string) (BlobstoreResult, error)
	StartUploadHandler     func(blob FileChunk) (BlobstoreResult, error)
	GetObjectInfoHandler   func(blob_id string, container_id string) (Blob, error)
	ReceiveChunkHandler    func(chunk FileChunk) error
)

func CreateContainerWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs CreateContainerArgs
	inputArgs.Decode(&decoder)
	response, err := CreateContainerHandler(inputArgs.ID)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func RemoveContainerWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs RemoveContainerArgs
	inputArgs.Decode(&decoder)
	response, err := RemoveContainerHandler(inputArgs.ID)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func RemoveObjectWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs RemoveObjectArgs
	inputArgs.Decode(&decoder)
	response, err := RemoveObjectHandler(inputArgs.ID, inputArgs.Container_id)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func ListObjectsWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs ListObjectsArgs
	inputArgs.Decode(&decoder)
	response, err := ListObjectsHandler(inputArgs.Container_id)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func UploadChunkWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var request FileChunk
	request.Decode(&decoder)
	err := UploadChunkHandler(request)
	if err != nil {
		return nil, err
	}
	return []byte{}, nil
}

func StartDownloadWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs StartDownloadArgs
	inputArgs.Decode(&decoder)
	response, err := StartDownloadHandler(inputArgs.Blob_id, inputArgs.Container_id, inputArgs.Chunk_size, inputArgs.Context)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func StartUploadWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var request FileChunk
	request.Decode(&decoder)
	response, err := StartUploadHandler(request)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func GetObjectInfoWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs GetObjectInfoArgs
	inputArgs.Decode(&decoder)
	response, err := GetObjectInfoHandler(inputArgs.Blob_id, inputArgs.Container_id)
	if err != nil {
		return nil, err
	}
	return msgpack.ToBytes(&response)
}

func ReceiveChunkWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var request FileChunk
	request.Decode(&decoder)
	err := ReceiveChunkHandler(request)
	if err != nil {
		return nil, err
	}
	return []byte{}, nil
}

type CreateContainerArgs struct {
	ID string
}

func DecodeCreateContainerArgsNullable(decoder *msgpack.Decoder) (*CreateContainerArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeCreateContainerArgs(decoder)
	return &decoded, err
}

func DecodeCreateContainerArgs(decoder *msgpack.Decoder) (CreateContainerArgs, error) {
	var o CreateContainerArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *CreateContainerArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "id":
			o.ID, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *CreateContainerArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)

	return nil
}

type RemoveContainerArgs struct {
	ID string
}

func DecodeRemoveContainerArgsNullable(decoder *msgpack.Decoder) (*RemoveContainerArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeRemoveContainerArgs(decoder)
	return &decoded, err
}

func DecodeRemoveContainerArgs(decoder *msgpack.Decoder) (RemoveContainerArgs, error) {
	var o RemoveContainerArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *RemoveContainerArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "id":
			o.ID, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *RemoveContainerArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)

	return nil
}

type RemoveObjectArgs struct {
	ID           string
	Container_id string
}

func DecodeRemoveObjectArgsNullable(decoder *msgpack.Decoder) (*RemoveObjectArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeRemoveObjectArgs(decoder)
	return &decoded, err
}

func DecodeRemoveObjectArgs(decoder *msgpack.Decoder) (RemoveObjectArgs, error) {
	var o RemoveObjectArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *RemoveObjectArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "id":
			o.ID, err = decoder.ReadString()
		case "container_id":
			o.Container_id, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *RemoveObjectArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(2)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)
	encoder.WriteString("container_id")
	encoder.WriteString(o.Container_id)

	return nil
}

type ListObjectsArgs struct {
	Container_id string
}

func DecodeListObjectsArgsNullable(decoder *msgpack.Decoder) (*ListObjectsArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeListObjectsArgs(decoder)
	return &decoded, err
}

func DecodeListObjectsArgs(decoder *msgpack.Decoder) (ListObjectsArgs, error) {
	var o ListObjectsArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *ListObjectsArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "container_id":
			o.Container_id, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *ListObjectsArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("container_id")
	encoder.WriteString(o.Container_id)

	return nil
}

type StartDownloadArgs struct {
	Blob_id      string
	Container_id string
	Chunk_size   uint64
	Context      *string
}

func DecodeStartDownloadArgsNullable(decoder *msgpack.Decoder) (*StartDownloadArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeStartDownloadArgs(decoder)
	return &decoded, err
}

func DecodeStartDownloadArgs(decoder *msgpack.Decoder) (StartDownloadArgs, error) {
	var o StartDownloadArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *StartDownloadArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "blob_id":
			o.Blob_id, err = decoder.ReadString()
		case "container_id":
			o.Container_id, err = decoder.ReadString()
		case "chunk_size":
			o.Chunk_size, err = decoder.ReadUint64()
		case "context":
			isNil, err := decoder.IsNextNil()
			if err == nil {
				if isNil {
					o.Context = nil
				} else {
					var nonNil string
					nonNil, err = decoder.ReadString()
					o.Context = &nonNil
				}
			}
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *StartDownloadArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(4)
	encoder.WriteString("blob_id")
	encoder.WriteString(o.Blob_id)
	encoder.WriteString("container_id")
	encoder.WriteString(o.Container_id)
	encoder.WriteString("chunk_size")
	encoder.WriteUint64(o.Chunk_size)
	encoder.WriteString("context")
	if o.Context == nil {
		encoder.WriteNil()
	} else {
		encoder.WriteString(*o.Context)
	}

	return nil
}

type GetObjectInfoArgs struct {
	Blob_id      string
	Container_id string
}

func DecodeGetObjectInfoArgsNullable(decoder *msgpack.Decoder) (*GetObjectInfoArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeGetObjectInfoArgs(decoder)
	return &decoded, err
}

func DecodeGetObjectInfoArgs(decoder *msgpack.Decoder) (GetObjectInfoArgs, error) {
	var o GetObjectInfoArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *GetObjectInfoArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "blob_id":
			o.Blob_id, err = decoder.ReadString()
		case "container_id":
			o.Container_id, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *GetObjectInfoArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(2)
	encoder.WriteString("blob_id")
	encoder.WriteString(o.Blob_id)
	encoder.WriteString("container_id")
	encoder.WriteString(o.Container_id)

	return nil
}

type BlobstoreResult struct {
	Success bool
	Error   *string
}

func DecodeBlobstoreResultNullable(decoder *msgpack.Decoder) (*BlobstoreResult, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeBlobstoreResult(decoder)
	return &decoded, err
}

func DecodeBlobstoreResult(decoder *msgpack.Decoder) (BlobstoreResult, error) {
	var o BlobstoreResult
	err := o.Decode(decoder)
	return o, err
}

func (o *BlobstoreResult) Decode(decoder *msgpack.Decoder) error {
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
		case "success":
			o.Success, err = decoder.ReadBool()
		case "error":
			isNil, err := decoder.IsNextNil()
			if err == nil {
				if isNil {
					o.Error = nil
				} else {
					var nonNil string
					nonNil, err = decoder.ReadString()
					o.Error = &nonNil
				}
			}
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *BlobstoreResult) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(2)
	encoder.WriteString("success")
	encoder.WriteBool(o.Success)
	encoder.WriteString("error")
	if o.Error == nil {
		encoder.WriteNil()
	} else {
		encoder.WriteString(*o.Error)
	}

	return nil
}

type FileChunk struct {
	SequenceNo uint64
	Container  Container
	ID         string
	TotalBytes uint64
	ChunkSize  uint64
	Context    *string
	ChunkBytes []byte
}

func DecodeFileChunkNullable(decoder *msgpack.Decoder) (*FileChunk, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeFileChunk(decoder)
	return &decoded, err
}

func DecodeFileChunk(decoder *msgpack.Decoder) (FileChunk, error) {
	var o FileChunk
	err := o.Decode(decoder)
	return o, err
}

func (o *FileChunk) Decode(decoder *msgpack.Decoder) error {
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
		case "sequenceNo":
			o.SequenceNo, err = decoder.ReadUint64()
		case "container":
			o.Container, err = DecodeContainer(decoder)
		case "id":
			o.ID, err = decoder.ReadString()
		case "totalBytes":
			o.TotalBytes, err = decoder.ReadUint64()
		case "chunkSize":
			o.ChunkSize, err = decoder.ReadUint64()
		case "context":
			isNil, err := decoder.IsNextNil()
			if err == nil {
				if isNil {
					o.Context = nil
				} else {
					var nonNil string
					nonNil, err = decoder.ReadString()
					o.Context = &nonNil
				}
			}
		case "chunkBytes":
			o.ChunkBytes, err = decoder.ReadByteArray()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *FileChunk) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(7)
	encoder.WriteString("sequenceNo")
	encoder.WriteUint64(o.SequenceNo)
	encoder.WriteString("container")
	o.Container.Encode(encoder)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)
	encoder.WriteString("totalBytes")
	encoder.WriteUint64(o.TotalBytes)
	encoder.WriteString("chunkSize")
	encoder.WriteUint64(o.ChunkSize)
	encoder.WriteString("context")
	if o.Context == nil {
		encoder.WriteNil()
	} else {
		encoder.WriteString(*o.Context)
	}
	encoder.WriteString("chunkBytes")
	encoder.WriteByteArray(o.ChunkBytes)

	return nil
}

type Container struct {
	ID string
}

func DecodeContainerNullable(decoder *msgpack.Decoder) (*Container, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeContainer(decoder)
	return &decoded, err
}

func DecodeContainer(decoder *msgpack.Decoder) (Container, error) {
	var o Container
	err := o.Decode(decoder)
	return o, err
}

func (o *Container) Decode(decoder *msgpack.Decoder) error {
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
		case "id":
			o.ID, err = decoder.ReadString()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *Container) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)

	return nil
}

type ContainerList struct {
	Containers []Container
}

func DecodeContainerListNullable(decoder *msgpack.Decoder) (*ContainerList, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeContainerList(decoder)
	return &decoded, err
}

func DecodeContainerList(decoder *msgpack.Decoder) (ContainerList, error) {
	var o ContainerList
	err := o.Decode(decoder)
	return o, err
}

func (o *ContainerList) Decode(decoder *msgpack.Decoder) error {
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
		case "containers":
			listSize, err := decoder.ReadArraySize()
			if err != nil {
				return err
			}
			o.Containers = make([]Container, 0, listSize)
			for listSize > 0 {
				listSize--
				var nonNilItem Container
				nonNilItem, err = DecodeContainer(decoder)
				if err != nil {
					return err
				}
				o.Containers = append(o.Containers, nonNilItem)
			}
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *ContainerList) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("containers")
	encoder.WriteArraySize(uint32(len(o.Containers)))
	for _, v := range o.Containers {
		v.Encode(encoder)
	}

	return nil
}

type Blob struct {
	ID        string
	Container Container
	ByteSize  uint64
}

func DecodeBlobNullable(decoder *msgpack.Decoder) (*Blob, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeBlob(decoder)
	return &decoded, err
}

func DecodeBlob(decoder *msgpack.Decoder) (Blob, error) {
	var o Blob
	err := o.Decode(decoder)
	return o, err
}

func (o *Blob) Decode(decoder *msgpack.Decoder) error {
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
		case "id":
			o.ID, err = decoder.ReadString()
		case "container":
			o.Container, err = DecodeContainer(decoder)
		case "byteSize":
			o.ByteSize, err = decoder.ReadUint64()
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *Blob) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(3)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)
	encoder.WriteString("container")
	o.Container.Encode(encoder)
	encoder.WriteString("byteSize")
	encoder.WriteUint64(o.ByteSize)

	return nil
}

type BlobList struct {
	Blobs []Blob
}

func DecodeBlobListNullable(decoder *msgpack.Decoder) (*BlobList, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeBlobList(decoder)
	return &decoded, err
}

func DecodeBlobList(decoder *msgpack.Decoder) (BlobList, error) {
	var o BlobList
	err := o.Decode(decoder)
	return o, err
}

func (o *BlobList) Decode(decoder *msgpack.Decoder) error {
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
		case "blobs":
			listSize, err := decoder.ReadArraySize()
			if err != nil {
				return err
			}
			o.Blobs = make([]Blob, 0, listSize)
			for listSize > 0 {
				listSize--
				var nonNilItem Blob
				nonNilItem, err = DecodeBlob(decoder)
				if err != nil {
					return err
				}
				o.Blobs = append(o.Blobs, nonNilItem)
			}
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *BlobList) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("blobs")
	encoder.WriteArraySize(uint32(len(o.Blobs)))
	for _, v := range o.Blobs {
		v.Encode(encoder)
	}

	return nil
}

type StreamRequest struct {
	ID        string
	Container Container
	ChunkSize uint64
	Context   *string
}

func DecodeStreamRequestNullable(decoder *msgpack.Decoder) (*StreamRequest, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeStreamRequest(decoder)
	return &decoded, err
}

func DecodeStreamRequest(decoder *msgpack.Decoder) (StreamRequest, error) {
	var o StreamRequest
	err := o.Decode(decoder)
	return o, err
}

func (o *StreamRequest) Decode(decoder *msgpack.Decoder) error {
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
		case "id":
			o.ID, err = decoder.ReadString()
		case "container":
			o.Container, err = DecodeContainer(decoder)
		case "chunkSize":
			o.ChunkSize, err = decoder.ReadUint64()
		case "context":
			isNil, err := decoder.IsNextNil()
			if err == nil {
				if isNil {
					o.Context = nil
				} else {
					var nonNil string
					nonNil, err = decoder.ReadString()
					o.Context = &nonNil
				}
			}
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *StreamRequest) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(4)
	encoder.WriteString("id")
	encoder.WriteString(o.ID)
	encoder.WriteString("container")
	o.Container.Encode(encoder)
	encoder.WriteString("chunkSize")
	encoder.WriteUint64(o.ChunkSize)
	encoder.WriteString("context")
	if o.Context == nil {
		encoder.WriteNil()
	} else {
		encoder.WriteString(*o.Context)
	}

	return nil
}

type Transfer struct {
	BlobID      string
	Container   Container
	ChunkSize   uint64
	TotalSize   uint64
	TotalChunks uint64
	Context     *string
}

func DecodeTransferNullable(decoder *msgpack.Decoder) (*Transfer, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeTransfer(decoder)
	return &decoded, err
}

func DecodeTransfer(decoder *msgpack.Decoder) (Transfer, error) {
	var o Transfer
	err := o.Decode(decoder)
	return o, err
}

func (o *Transfer) Decode(decoder *msgpack.Decoder) error {
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
		case "blobId":
			o.BlobID, err = decoder.ReadString()
		case "container":
			o.Container, err = DecodeContainer(decoder)
		case "chunkSize":
			o.ChunkSize, err = decoder.ReadUint64()
		case "totalSize":
			o.TotalSize, err = decoder.ReadUint64()
		case "totalChunks":
			o.TotalChunks, err = decoder.ReadUint64()
		case "context":
			isNil, err := decoder.IsNextNil()
			if err == nil {
				if isNil {
					o.Context = nil
				} else {
					var nonNil string
					nonNil, err = decoder.ReadString()
					o.Context = &nonNil
				}
			}
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *Transfer) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(6)
	encoder.WriteString("blobId")
	encoder.WriteString(o.BlobID)
	encoder.WriteString("container")
	o.Container.Encode(encoder)
	encoder.WriteString("chunkSize")
	encoder.WriteUint64(o.ChunkSize)
	encoder.WriteString("totalSize")
	encoder.WriteUint64(o.TotalSize)
	encoder.WriteString("totalChunks")
	encoder.WriteUint64(o.TotalChunks)
	encoder.WriteString("context")
	if o.Context == nil {
		encoder.WriteNil()
	} else {
		encoder.WriteString(*o.Context)
	}

	return nil
}
