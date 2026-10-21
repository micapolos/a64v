#include <stdio.h>

#include "emitter.h"

void a64_emitter_init(a64_emitter *emitter) {
  emitter->tag = a64_emitter_tag_init;
}

void a64_emitter_emit_a64(a64_emitter * emitter, a64 op) {
  if (a64_emitter_is_error(emitter)) return;
  if (a64_is_nop(op)) {
    printf("NOP\n");
  } else if (a64_is_ret(op)) {
    printf("RET\n");
  } else {
    printf("A64_ERROR\n");
    emitter->tag = a64_emitter_tag_error;
  }
}

void a64_emitter_emit_va64(a64_emitter *emitter, va64 op) {
  if (emitter->tag == a64_emitter_tag_error) return;
  switch (va64_get_tag(op)) {
  case VA64_TAG_STEP:
    printf("STEP %u\n", va64_get_step_offset(op));
    break;
  case VA64_TAG_FN:
    printf("FN %d, %d\n", va64_get_fn_arity(op), va64_get_fn_ret(op));
    break;
  default:
    printf("VA64_ERROR\n");
    emitter->tag = a64_emitter_tag_error;
  }
}
