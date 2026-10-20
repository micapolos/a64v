#include <stdio.h>
#include <assert.h>

#include "a64.h"

int main() {
  assert(a64_is_nop(a64_nop()));
  return 0;
}
