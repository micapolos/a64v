#pragma once

#include <stdint.h>

typedef uint32_t va64;

typedef enum {
  VA64_TAG_OFS = 0,
  VA64_TAG_FN = 1,
} va64_tag;

#define VA64_OFS(n) ((VA64_TAG_OFS << 26) | n)
#define VA64_FN(arity, ret) ((VA64_TAG_FN << 26) | (arity << 23) | (ret << 22))

static inline unsigned va64_get_tag(va64 op) {
  return op >> 26;
}

static inline unsigned va64_get_fn_arity(va64 op) {
  return (op >> 23) & 0b111;
}

static inline unsigned va64_get_fn_ret(va64 op) {
  return (op >> 22) & 0b1;
}
