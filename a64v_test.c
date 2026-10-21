#include <stdio.h>
#include <assert.h>

#include "a64.h"
#include "va64.h"
#include "a64v.h"

int main() {
  static const a64 a64_code[] = {
    a64_nop(),
    a64_nop(),
    a64_nop(),
    a64_ret(),
  };

  static const va64 va64_code[] = {
    va64_fn(3, 1),
  };

  a64v_state state;
  a64v_state_init(
    &state,
    a64_code, 3,
    va64_code, 1);

  assert(state.tag == a64v_state_tag_start);
  assert(state.a64_end - state.a64_ptr == 3);
  assert(state.va64_end - state.va64_ptr == 1);

  return 0;
}
