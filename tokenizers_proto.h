#include <stddef.h>
#include <stdint.h>

/**
 * # Safety
 * Function must be called after a function that has an output.
 * With the same address and len the output was pointed to.
 */
void lib_tokenizers_free_buffer(uint8_t *ptr, size_t len);

typedef struct lib_tokenizers_PreTokenizedString PreTokenizedString;

int32_t lib_tokenizers_get_splits(PreTokenizedString *instance_ptr,
                                  const uint8_t *ptr,
                                  size_t len,
                                  uint8_t **out_ptr,
                                  size_t *out_len);

int32_t lib_tokenizers_new_pipeline_string(PreTokenizedString **instance_ptr,
                                           const uint8_t *ptr,
                                           size_t len,
                                           uint8_t **out_ptr,
                                           size_t *out_len);

void lib_tokenizers_free_pipeline_string(PreTokenizedString *ptr);

typedef struct lib_tokenizers_NormalizerWrapper NormalizerWrapper;

int32_t lib_tokenizers_normalize(NormalizerWrapper *instance_ptr,
                                 const uint8_t *ptr,
                                 size_t len,
                                 uint8_t **out_ptr,
                                 size_t *out_len);

int32_t lib_tokenizers_new_normalizer_wrapper(NormalizerWrapper **instance_ptr,
                                              const uint8_t *ptr,
                                              size_t len,
                                              uint8_t **out_ptr,
                                              size_t *out_len);

void lib_tokenizers_free_normalizer_wrapper(NormalizerWrapper *ptr);

typedef struct lib_tokenizers_PreTokenizerWrapper PreTokenizerWrapper;

int32_t lib_tokenizers_pre_tokenize(PreTokenizerWrapper *instance_ptr,
                                    const uint8_t *ptr,
                                    size_t len,
                                    uint8_t **out_ptr,
                                    size_t *out_len);

int32_t lib_tokenizers_new_pre_tokenizer_wrapper(PreTokenizerWrapper **instance_ptr,
                                                 const uint8_t *ptr,
                                                 size_t len,
                                                 uint8_t **out_ptr,
                                                 size_t *out_len);

void lib_tokenizers_free_pre_tokenizer_wrapper(PreTokenizerWrapper *ptr);

typedef struct lib_tokenizers_Tokenizer Tokenizer;

int32_t lib_tokenizers_tokenizer_from_file(Tokenizer **instance_ptr,
                                           const uint8_t *ptr,
                                           size_t len,
                                           uint8_t **out_ptr,
                                           size_t *out_len);

int32_t lib_tokenizers_encode(const Tokenizer *instance_ptr,
                              const uint8_t *ptr,
                              size_t len,
                              uint8_t **out_ptr,
                              size_t *out_len);

int32_t lib_tokenizers_decode(const Tokenizer *instance_ptr,
                              const uint8_t *ptr,
                              size_t len,
                              uint8_t **out_ptr,
                              size_t *out_len);

void lib_tokenizers_free_tokenizer(Tokenizer *ptr);

int32_t lib_tokenizers_tokenizer_from_train(Tokenizer **instance_ptr,
                                            const uint8_t *ptr,
                                            size_t len,
                                            uint8_t **out_ptr,
                                            size_t *out_len);
