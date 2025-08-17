# Main goal

Expose [huggingface/tokenizers](https://github.com/huggingface/tokenizers) as a dynamic library with a minimal C ABI using only primitive types (i32, raw pointers) and [Protocol Buffers](https://github.com/protocolbuffers/protobuf) for structured input/output.

## Exposed Functions

Below is a summary of the exported functions, their signatures, and usage:

### Buffer Management

- `free_buffer(*ptr, len: usize)`
    - Frees a buffer returned by functions with `**out_ptr`.
    - **Implementation:** `src/buffer_utils.rs`

### Normalizer Functions

- `normalize(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Normalizes input using a normalizer instance.
    - **Input:** `protos/normalizers/normalizer.proto` (`NormalizeParams`)
    - **Output:** `protos/normalizers/normalizer.proto` (`NormalizeResult`)
    - **Implementation:** `src/normalizers.rs`

- `new_normalizer_wrapper(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Creates a new normalizer instance.
    - **Input:** `protos/normalizers/normalizer_intance.proto` (`NormalizerWrapperParams`)
    - **Implementation:** `src/normalizers.rs`

- `free_normalizer_wrapper(*instance_ptr)`
    - Frees a normalizer instance.
    - **Implementation:** `src/normalizers.rs`

### Pre-Tokenizer Functions

- `pre_tokenize(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Pre-tokenizes input using a pre-tokenizer instance.
    - **Input:** `protos/pre_tokenizers/pre_tokenizer.proto` (`PreTokenizeParams`)
    - **Output:** `protos/pre_tokenizers/pre_tokenizer.proto` (`PreTokenizeResult`)
    - **Implementation:** `src/pre_tokenizers.rs`

- `new_pre_tokenizer_wrapper(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Creates a new pre-tokenizer instance.
    - **Input:** `protos/pre_tokenizers/pre_tokenizer_intance.proto` (`PreTokenizerWrapperParams`)
    - **Implementation:** `src/pre_tokenizers.rs`

- `free_pre_tokenizer_wrapper(*instance_ptr)`
    - Frees a pre-tokenizer instance.
    - **Implementation:** `src/pre_tokenizers.rs`

### Tokenizer Functions

- `tokenizer_from_file(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Loads a tokenizer from a file.
    - **Input:** `protos/tokenizer.proto` (`TokenizerFromFile`)
    - **Implementation:** `src/tokenizer.rs`

- `encode(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Encodes input using a tokenizer instance.
    - **Input:** `protos/tokenizer.proto` (`EncodeParams`)
    - **Output:** `protos/tokenizer.proto` (`EncodeResult`)
    - **Implementation:** `src/tokenizer.rs`

- `decode(*instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Decodes token IDs to a string using a tokenizer instance.
    - **Input:** `protos/tokenizer.proto` (`DecodeParams`)
    - **Output:** `protos/tokenizer.proto` (`DecodeResult`)
    - **Implementation:** `src/tokenizer.rs`

- `free_tokenizer(*instance_ptr)`
    - Frees a tokenizer instance.
    - **Implementation:** `src/tokenizer.rs`

### Trainer Functions

- `tokenizer_from_train(**instance_ptr, *ptr, len, **out_ptr, *out_len) -> i32`
    - Trains and creates a tokenizer.
    - **Input:** `protos/trainers/trainer.proto` (`TrainParams`)
    - **Implementation:** `src/trainer.rs`

---

## Parameter Explanation

- `**instance_ptr`: Double pointer to an internal object (tokenizer, normalizer, or pre-tokenizer). Used to create and return a new instance.  
    **Direction:** OUT

- `*instance_ptr`: Pointer to an internal object. Used to reference an existing instance for operations.  
    **Direction:** IN

- `**out_ptr`: Double pointer for returning a buffer (Protobuf encoded).  
    **Direction:** OUT

- `*out_len`: Pointer to the length of the output buffer.  
    **Direction:** OUT

- `*ptr`: Pointer to an input buffer (Protobuf encoded parameters).  
    **Direction:** IN

- `len`: Length of the input buffer.  
    **Direction:** IN

> Functions returning `i32` indicate success or error codes. If `**out_ptr` is present, errors are returned as a Protobuf-encoded `Error` message or null.
