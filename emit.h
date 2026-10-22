#pragma once

uint32_t *a64_emit_ptr;

static inline void emit(uint32_t op) {
  *a64_emit_ptr++ = op;
}
