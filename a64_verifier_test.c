#include <stdio.h>
#include <assert.h>

#include "a64_verifier.h"

int main() {
  a64_verifier verifier;
  a64_verifier_init(&verifier);
  assert(verifier.tag == a64_verifier_tag_init);
  return 0;
}
