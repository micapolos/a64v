#include <stdio.h>
#include <assert.h>

#include "a64.h"
#include "va64.h"
#include "a64v.h"

int main() {
  static a64 a64_code[] = {
    a64_nop(),
    a64_nop(),
    a64_nop(),
  };

  static va64 va64_code[] = {
    va64_fn(3, 1),
  };

  a64v_state state;
  a64v_state_init(
    &state,
    a64_code, 3,
    va64_code, 1);

  return 0;
}
