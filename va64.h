#pragma once

#include <stdint.h>

typedef uint32_t va64;

typedef enum : unsigned {
  va64_tag_ofs = 0,
  va64_tag_fn = 1,
} va64_tag_t;

static inline va64 va64_fn(unsigned arity, unsigned ret) {
  return (va64_tag_fn << 26) | (arity << 23) | (ret << 22);
}

static inline unsigned va64_tag(va64 op) {
  return op >> 26;
}

static inline bool va64_is_fn(va64 op) {
  return va64_tag(op) == va64_tag_fn;
}

static inline unsigned va64_fn_arity(va64 op) {
  return (op >> 23) & 0b111;
}

static inline unsigned va64_fn_ret(va64 op) {
  return (op >> 22) & 0b1;
}
