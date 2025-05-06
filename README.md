# Main goal

Expose [huggingface/tokenizers](https://github.com/huggingface/tokenizers) as a dynamic library with a minimal C ABI using only primitive types (i32, raw pointers) and [Protocol Buffers](https://github.com/protocolbuffers/protobuf) for structured input/output.

## Exposed functions

Pseudocode of the exported functions

`free_buffer(*ptr, len: usize)`

\\\ After a call of a function that has `**out_ptr` this function must be called to free the result buffer pointed by `*out_ptr`.

\\\ Implementation: src/buffer_utils.rs

`normalize(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/normalizers.rs

\\\ Input buffer: protos/normalizers/normalizer.proto(NormalizeParams)

\\\ Output buffer: protos/normalizers/normalizer.proto(NormalizeResult)

`new_normalizer_wrapper(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/normalizers.rs

\\\ Input buffer: protos/normalizers/normalizer_intance.proto(NormalizerWrapperParams)

`free_normalizer_wrapper(*instance_ptr)`

\\\ Implementation: src/normalizers.rs

`pre_tokenize(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/pre_tokenizers.rs

\\\ Input buffer. protos/pre_tokenizers/pre_tokenizer.proto(PreTokenizeParams)

\\\ Output buffer. protos/pre_tokenizers/pre_tokenizer.proto(PreTokenizeResult)

`new_pre_tokenizer_wrapper(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/pre_tokenizers.rs

\\\ Input buffer: protos/pre_tokenizers/pre_tokenizer_intance.proto(PreTokenizerWrapperParams)

`free_pre_tokenizer_wrapper(*instance_ptr)`

\\\ Implementation: src/pre_tokenizers.rs

`tokenizer_from_file(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/tokenizer.rs

\\\ Input buffer: protos/tokenizer.proto(TokenizerFromFile)

`encode(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/tokenizer.rs

\\\ Input buffer: protos/tokenizer.proto(EncodeParams)

\\\ Output buffer: protos/tokenizer.proto(EncodeResult)

`decode(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/tokenizer.rs

\\\ Input buffer: protos/tokenizer.proto(DecodeParams)

\\\ Output buffer: protos/tokenizer.proto(DecodeResult)

`free_tokenizer(*instance_ptr)`

\\\ Implementation: src/tokenizer.rs

`tokenizer_from_train(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`

\\\ Implementation: src/trainer.rs

\\\ Input buffer: protos/trainers/trainer.proto(TrainParams)

### Explanation

- `**instance_ptr`. A double pointer to an instance of an internal object (e.g., a tokenizer, normalizer, or pre-tokenizer). Used to initialize a pointer to a new instance of the internal object.

    Direction: OUT (output).

- `*instance_ptr`. A pointer to an instance of an internal object (e.g., a tokenizer, normalizer, or pre-tokenizer). Used to identify and operate on a specific object created by the DLL.

    Direction: IN (input).

    Example: You pass this pointer to tell the library which tokenizer instance you’re calling encode on.

- `**out_ptr`. A double pointer used to return a buffer from the function. The content of the buffer uses Protobuf encoding.

    Direction: OUT (output).

    Example: A Protobuf message containing a string decoded from tokenids.

- `*out_len`. Relates to `**out_ptr`, indicates the len of the buffer. Points to (unint|usize|an unsigned integer with the length of a word in the given architecture)

    Direction: OUT (output).

- `*ptr`. A pointer to a buffer with the calling params for a given function encoded in Protobuf format.

    Direction: IN (input).

    Example: A Protobuf message containing tokenids to be decoded into a string.

- `len`. Relates to `*ptr`, indicates the len of the buffer. Type: unint|usize|an unsigned integer with the length of a word in the given architecture.

    Direction: IN (input).

> Functions that return i32, the returned number indicates whether the calling was successful or not, and what kind of error happened. If the function has `**out_ptr`, it will point to null or a protos/error.proto(Error) encoded message.
