#pragma once

#include <stdint.h>

typedef uint32_t va64;

static inline va64 va64_fn(int arity, int ret) {
  return (0b000001 << 26) | (arity << 23) | (ret << 22);
}

static inline int va64_op(va64 v) {
  return v >>> 26;
}

static inline int va64_fn_arity(va64 v) {
  return (v >>> 23) & 0b111;
}

static inline int va64_fn_ret(va64 v) {
  return (v >>> 22) & 0b1;
}
