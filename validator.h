#pragma once

#include "a64.h"
#include "va64.h"

typedef enum : unsigned {
  a64_validator_tag_init = 0x0,
  a64_validator_tag_fn = 0x1,
  a64_validator_tag_error = 0xff,
} a64_validator_tag;

typedef struct {
  a64_validator_tag tag;
} a64_validator;

static inline bool a64_validator_is_error(a64_validator *validator) {
  return validator->tag == a64_validator_tag_error;
}

void a64_validator_init(a64_validator *);

// emit individual ops
void a64_validator_emit_a64(a64_validator *, a64);
void a64_validator_emit_va64(a64_validator *, va64);

// emit complete a64 and va64 ops
void a64_validator_emit(
  a64_validator *validator,
  const a64 *a64_ops, size_t a64_size,
  const va64 *va64_ops, size_t va64_size);
