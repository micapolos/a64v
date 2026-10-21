#include <stdio.h>
#include <assert.h>

#include "a64.h"
#include "va64.h"
#include "a64v.h"

int main() {
  static const a64 a64_code[] = {
    A64_NOP,
    A64_NOP,
    A64_RET,
  };

  static const va64 va64_code[] = {
    VA64_FN(3, 1),
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
