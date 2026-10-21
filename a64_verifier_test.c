#include <stdio.h>
#include <assert.h>

#include "a64.h"
#include "va64.h"
#include "a64_verifier.h"

int main() {
  a64_verifier verifier;

  {
    a64_verifier_init(&verifier);
    assert(verifier.tag == a64_verifier_tag_init);
  }

  {
    a64_verifier_init(&verifier);
    a64_verifier_emit_a64(&verifier, A64_NOP);
    assert(verifier.tag == a64_verifier_tag_error);
  }

  {
    a64_verifier_init(&verifier);
    a64_verifier_emit_va64(&verifier, VA64_OFS(0));
    assert(verifier.tag == a64_verifier_tag_error);
  }

  return 0;
}
