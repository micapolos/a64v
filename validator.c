#include <stdio.h>

#include "validator.h"

void a64_validator_init(a64_validator *validator) {
  validator->tag = a64_validator_tag_init;
}

void a64_validator_emit_a64(a64_validator * validator, a64 op) {
  switch (validator->tag) {
  case a64_validator_tag_error:
    break;
  case a64_validator_tag_fn:
    if (a64_is_nop(op)) {
      // Nothing to update
    } else if (a64_is_ret(op)) {
      validator->tag = a64_validator_tag_init;
    } else {
      validator->tag = a64_validator_tag_error;
    }
    break;
  default:
    validator->tag = a64_validator_tag_error;
  }
}

void a64_validator_emit_va64(a64_validator *validator, va64 op) {
  switch (validator->tag) {
  case a64_validator_tag_error:
    break;
  case a64_validator_tag_init:
    switch (va64_get_tag(op)) {
    case VA64_TAG_FN:
      if (va64_get_fn_arity(op) != 0) {
        validator->tag = a64_validator_tag_error;
      } else if (va64_get_fn_arity(op) != 0) {
        validator->tag = a64_validator_tag_error;
      } else {
        validator->tag = a64_validator_tag_fn;
      }
      break;
    default:
      validator->tag = a64_validator_tag_error;
    }
  default:
    validator->tag = a64_validator_tag_error;
  }
}

void a64_validator_emit(
    a64_validator *validator,
    const a64 *a64_ops, size_t a64_size,
    const va64 *va64_ops, size_t va64_size) {
  a64* a64_ptr = (a64*)a64_ops;
  a64* a64_end = a64_ptr + (a64_size >> 2);

  va64* va64_ptr = (va64*)va64_ops;
  va64* va64_end = a64_ptr + (va64_size >> 2);

  unsigned a64_offset = 0;
  while (a64_ptr != a64_end) {
    while (a64_offset == 0 && va64_ptr != va64_end) {
      unsigned va64_op = *va64_ptr;
      if (va64_get_tag(va64_op) == VA64_TAG_STEP) {
        a64_offset += va64_get_step_offset(va64_op);
        va64_ptr++;
      } else {
        a64_validator_emit_va64(validator, va64_op);
        continue;
      }
    }

    while (a64_offset != 0 && a64_ptr != a64_end) {
      a64 a64_op = *a64_ptr;
      a64_validator_emit_a64(validator, a64_op);
      a64_ptr++;
      a64_offset--;
    }
  }
}
