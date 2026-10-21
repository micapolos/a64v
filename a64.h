#pragma once

#include <stdint.h>

typedef uint32_t a64;

#define a64_nop() ((a64) 0b110'101'01000000110010'0000'000'11111)
#define a64_is_nop(op) ((op) == a64_nop())
