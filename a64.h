#pragma once

#include <stdint.h>

typedef uint32_t a64;

static inline a64 a64_nop() {
  return 0b110'101'01000000110010'0000'000'11111;
}

static inline bool a64_is_nop(a64 x) {
  return x == 0b110'101'01000000110010'0000'000'11111;
}
