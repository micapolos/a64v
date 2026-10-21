#pragma once

#include "a64.h"
#include "va64.h"

typedef enum : unsigned {
  a64_emitter_tag_init = 0x0,
  a64_emitter_tag_fn = 0x1,
  a64_emitter_tag_error = 0xff,
} a64_emitter_tag;

typedef struct {
  a64_emitter_tag tag;
} a64_emitter;

static inline bool a64_emitter_is_error(a64_emitter *emitter) {
  return emitter->tag == a64_emitter_tag_error;
}

void a64_emitter_init(a64_emitter *);
void a64_emitter_emit_a64(a64_emitter *, a64);
void a64_emitter_emit_va64(a64_emitter *, va64);
