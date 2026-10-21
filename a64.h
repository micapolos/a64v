#pragma once

#include <stdint.h>

typedef uint32_t a64;

#define A64_NOP ((a64) 0b110'101'01000000110010'0000'000'11111)
#define A64_RET_R(r) ((a64) (0b110'101'1'0'0'10'11111'0000'0'0 << 10) | (r << 5) | 0b00000)
#define A64_RET A64_RET_R(30)

static inline bool a64_is_nop(a64 op) {
  return op == A64_NOP;
}

static inline bool a64_is_ret(a64 op) {
  return op == A64_RET;
}
