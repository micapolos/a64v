#import "a64_verifier.h"

void a64_verifier_init(a64_verifier *verifier) {
  verifier->tag = a64_verifier_tag_init;
}

void a64_verifier_emit_a64(a64_verifier *, a64) {
}

void a64_verifier_emit_va64(a64_verifier *, va64) {
}
