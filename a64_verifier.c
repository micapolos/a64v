#import "a64_verifier.h"

void a64_verifier_init(a64_verifier *verifier) {
  verifier->tag = a64_verifier_tag_init;
}

void a64_verifier_emit_a64(a64_verifier * verifier, a64) {
  if (a64_verifier_is_error(verifier)) return;
  verifier->tag = a64_verifier_tag_error;
}

void a64_verifier_emit_va64(a64_verifier *verifier, va64) {
  if (verifier->tag == a64_verifier_tag_error) return;
  verifier->tag = a64_verifier_tag_error;
}
