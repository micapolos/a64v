#pragma once

#include <stdint.h>

typedef uint32_t a64;

static inline a64 a64_nop() {
  return 0b110'101'01000000110010'0000'000'11111;
}
