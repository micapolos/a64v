#include <stdio.h>

#include "a64.h"
#include "va64.h"

int main() {
  printf("nop: 0x%08X\n", a64_nop());
  printf("fn: 0x%08X\n", va64_fn(3, 0));
  return 0;
}
