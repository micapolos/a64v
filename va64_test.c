#include <assert.h>

#include "va64.h"

int main() {
  {
    va64 op = VA64_FN(3, 1);
    assert(va64_get_tag(op) == VA64_TAG_FN);
    assert(va64_get_fn_arity(op) == 3);
    assert(va64_get_fn_ret(op) == 1);
  }
  return 0;
}
