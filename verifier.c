#include <stdio.h>

#include "verifier.h"

void a64_verifier_init(a64_verifier *verifier) {
  verifier->tag = a64_verifier_tag_init;
}

void a64_verifier_emit_a64(a64_verifier * verifier, a64 op) {
  if (a64_verifier_is_error(verifier)) return;
  if (a64_is_nop(op)) {
    printf("NOP\n");
  } else if (a64_is_ret(op)) {
    printf("RET\n");
  } else {
    verifier->tag = a64_verifier_tag_error;
  }
}

void a64_verifier_emit_va64(a64_verifier *verifier, va64 op) {
  if (verifier->tag == a64_verifier_tag_error) return;
  switch (va64_get_tag(op)) {
  case VA64_TAG_STEP:
    printf("STEP\n");
    break;
  case VA64_TAG_FN:
    printf("FN\n");
    break;
  default:
    verifier->tag = a64_verifier_tag_error;
  }
}
