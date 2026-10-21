#pragma once

#include "a64.h"
#include "va64.h"

typedef enum : unsigned {
  a64_verifier_tag_init = 0x0,
  a64_verifier_tag_fn = 0x1,
  a64_verifier_tag_error = 0xff,
} a64_verifier_tag;

typedef struct {
  a64_verifier_tag tag;
} a64_verifier;

static inline bool a64_verifier_is_error(a64_verifier *verifier) {
  return verifier->tag == a64_verifier_tag_error;
}

void a64_verifier_init(a64_verifier *);
void a64_verifier_emit_a64(a64_verifier *, a64);
void a64_verifier_emit_va64(a64_verifier *, va64);
