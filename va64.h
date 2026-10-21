#pragma once

#include <stdint.h>

typedef uint32_t va64;

#define va64_tag_ofs 0
#define va64_tag_fn 1

#define va64_ofs(n) ((va64_tag_ofs << 26) | n)
#define va64_fn(arity, ret) ((va64_tag_fn << 26) | (arity << 23) | (ret << 22))
#define va64_tag(op) ((op) >> 26)
#define va64_is_fn(op) (va64_tag(op) == va64_tag_fn)
#define va64_fn_arity(op) (((op) >> 23) & 0b111)
#define va64_fn_ret(op) (((op) >> 22) & 0b1)

