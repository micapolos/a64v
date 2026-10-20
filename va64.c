#include <stdio.h>

#include "a64.h"
#include "va64.h"

int main() {
  printf("nop: 0x%08X\n", a64_nop());
  printf("fn: 0x%08X\n", va64_fn(3, 0));
  printf("fn: 0x%08X\n", va64_op(va64_fn(3, 1)));
  printf("fn: 0x%08X\n", va64_fn_arity(va64_fn(3, 1)));
  printf("fn: 0x%08X\n", va64_fn_ret(va64_fn(3, 1)));
  return 0;
}
