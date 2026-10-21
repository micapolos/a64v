#include <assert.h>

#include "va64.h"

int main() {
  {
    va64 op = va64_fn(3, 1);
    assert(va64_tag(op) == va64_tag_fn);
    assert(va64_is_fn(op));
    assert(va64_fn_arity(op) == 3);
    assert(va64_fn_ret(op) == 1);
  }
  return 0;
}
