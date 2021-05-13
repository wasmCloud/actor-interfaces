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

// Create a container in a blobstore. Returns the container created if successful
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

// Remove a container from a blobstore
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

// Remove an object from a blobstore
func (h *Host) RemoveObject(id string, containerID string) (BlobstoreResult, error) {
	inputArgs := RemoveObjectArgs{
		ID:          id,
		ContainerID: containerID,
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

// Returns a list of blobs that are present in the specified container
func (h *Host) ListObjects(containerID string) (BlobList, error) {
	inputArgs := ListObjectsArgs{
		ContainerID: containerID,
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

// Upload a file chunk to a blobstore, which may only be part of a full file. This
// must be called AFTER the StartUpload operation. Chunks should be small, as
// memory over a few megabytes may exceed the wasm memory allocation.
func (h *Host) UploadChunk(chunk FileChunk) (BlobstoreResult, error) {
	inputArgs := UploadChunkArgs{
		Chunk: chunk,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return BlobstoreResult{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"UploadChunk",
		inputBytes,
	)
	if err != nil {
		return BlobstoreResult{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobstoreResult(&decoder)
}

// Issue a request to start a download from a blobstore. Chunks will be sent to the
// function that's registered with the ReceiveChunk operation.
func (h *Host) StartDownload(blobID string, containerID string, chunkSize uint64, context *string) (BlobstoreResult, error) {
	inputArgs := StartDownloadArgs{
		BlobID:      blobID,
		ContainerID: containerID,
		ChunkSize:   chunkSize,
		Context:     context,
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

// Begin the upload process with the first chunk of a full file. Subsequent chunks
// should be uploaded with the UploadChunk operation.
func (h *Host) StartUpload(chunk FileChunk) (BlobstoreResult, error) {
	inputArgs := StartUploadArgs{
		Chunk: chunk,
	}
	inputBytes, err := msgpack.ToBytes(&inputArgs)
	if err != nil {
		return BlobstoreResult{}, err
	}
	payload, err := wapc.HostCall(
		h.binding,
		"wasmcloud:blobstore",
		"StartUpload",
		inputBytes,
	)
	if err != nil {
		return BlobstoreResult{}, err
	}
	decoder := msgpack.NewDecoder(payload)
	return DecodeBlobstoreResult(&decoder)
}

// Retreives information about a blob
func (h *Host) GetObjectInfo(blobID string, containerID string) (Blob, error) {
	inputArgs := GetObjectInfoArgs{
		BlobID:      blobID,
		ContainerID: containerID,
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

type Handlers struct {
	// Defines a handler for incoming chunks forwarded by a wasmcloud:blobstore
	// provider. Chunks may not be received in order.
	ReceiveChunk func(chunk FileChunk) error
}

func (h Handlers) Register() {
	if h.ReceiveChunk != nil {
		receiveChunkHandler = h.ReceiveChunk
		wapc.RegisterFunction("ReceiveChunk", receiveChunkWrapper)
	}
}

var (
	receiveChunkHandler func(chunk FileChunk) error
)

func receiveChunkWrapper(payload []byte) ([]byte, error) {
	decoder := msgpack.NewDecoder(payload)
	var inputArgs ReceiveChunkArgs
	inputArgs.Decode(&decoder)
	err := receiveChunkHandler(inputArgs.Chunk)
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
	ID          string
	ContainerID string
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
			o.ContainerID, err = decoder.ReadString()
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
	encoder.WriteString(o.ContainerID)

	return nil
}

type ListObjectsArgs struct {
	ContainerID string
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
			o.ContainerID, err = decoder.ReadString()
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
	encoder.WriteString(o.ContainerID)

	return nil
}

type UploadChunkArgs struct {
	Chunk FileChunk
}

func DecodeUploadChunkArgsNullable(decoder *msgpack.Decoder) (*UploadChunkArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeUploadChunkArgs(decoder)
	return &decoded, err
}

func DecodeUploadChunkArgs(decoder *msgpack.Decoder) (UploadChunkArgs, error) {
	var o UploadChunkArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *UploadChunkArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "chunk":
			o.Chunk, err = DecodeFileChunk(decoder)
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *UploadChunkArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("chunk")
	o.Chunk.Encode(encoder)

	return nil
}

type StartDownloadArgs struct {
	BlobID      string
	ContainerID string
	ChunkSize   uint64
	Context     *string
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
			o.BlobID, err = decoder.ReadString()
		case "container_id":
			o.ContainerID, err = decoder.ReadString()
		case "chunk_size":
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

func (o *StartDownloadArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(4)
	encoder.WriteString("blob_id")
	encoder.WriteString(o.BlobID)
	encoder.WriteString("container_id")
	encoder.WriteString(o.ContainerID)
	encoder.WriteString("chunk_size")
	encoder.WriteUint64(o.ChunkSize)
	encoder.WriteString("context")
	if o.Context == nil {
		encoder.WriteNil()
	} else {
		encoder.WriteString(*o.Context)
	}

	return nil
}

type StartUploadArgs struct {
	Chunk FileChunk
}

func DecodeStartUploadArgsNullable(decoder *msgpack.Decoder) (*StartUploadArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeStartUploadArgs(decoder)
	return &decoded, err
}

func DecodeStartUploadArgs(decoder *msgpack.Decoder) (StartUploadArgs, error) {
	var o StartUploadArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *StartUploadArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "chunk":
			o.Chunk, err = DecodeFileChunk(decoder)
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *StartUploadArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("chunk")
	o.Chunk.Encode(encoder)

	return nil
}

type GetObjectInfoArgs struct {
	BlobID      string
	ContainerID string
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
			o.BlobID, err = decoder.ReadString()
		case "container_id":
			o.ContainerID, err = decoder.ReadString()
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
	encoder.WriteString(o.BlobID)
	encoder.WriteString("container_id")
	encoder.WriteString(o.ContainerID)

	return nil
}

type ReceiveChunkArgs struct {
	Chunk FileChunk
}

func DecodeReceiveChunkArgsNullable(decoder *msgpack.Decoder) (*ReceiveChunkArgs, error) {
	if isNil, err := decoder.IsNextNil(); isNil || err != nil {
		return nil, err
	}
	decoded, err := DecodeReceiveChunkArgs(decoder)
	return &decoded, err
}

func DecodeReceiveChunkArgs(decoder *msgpack.Decoder) (ReceiveChunkArgs, error) {
	var o ReceiveChunkArgs
	err := o.Decode(decoder)
	return o, err
}

func (o *ReceiveChunkArgs) Decode(decoder *msgpack.Decoder) error {
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
		case "chunk":
			o.Chunk, err = DecodeFileChunk(decoder)
		default:
			err = decoder.Skip()
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (o *ReceiveChunkArgs) Encode(encoder msgpack.Writer) error {
	if o == nil {
		encoder.WriteNil()
		return nil
	}
	encoder.WriteMapSize(1)
	encoder.WriteString("chunk")
	o.Chunk.Encode(encoder)

	return nil
}

// Represents a single chunk that may comprise part of a file or an entire file.
// The fields for sequence number, total bytes and chunk bytes should be used to
// determine the chunk order, as well as the optional context field.
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

// A container is a logical grouping of blobs, similar to a directory in a file
// system.
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

// A wrapper object around a list of containers.
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

// A blob is a representation of an object in a blobstore, similar to a file in a
// file system.
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

// A wrapper object around a list of blobs.
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

// Used to return success and error information for common blobstore operations
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
