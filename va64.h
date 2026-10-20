#pragma once

#include <stdint.h>

typedef uint32_t va64_t;

static const int va64_op_fn = 0x1;

static inline va64_t va64_fn(int arity, int ret) {
  return (va64_op_fn << 26) | (arity << 23) | (ret << 22);
}

static inline int va64_op(va64_t va64) {
  return va64 >> 26;
}

static inline int va64_fn_arity(va64_t va64) {
  return (va64 >> 23) & 0b111;
}

static inline int va64_fn_ret(va64_t va64) {
  return (va64 >> 22) & 0b1;
}
