#pragma once

typedef enum : unsigned {
  a64v_state_tag_start = 0x0,
  a64v_state_tag_fn = 0x1,
  a64v_state_tag_error = 0xff,
} a64v_state_tag;

typedef struct {
  a64 *a64_ptr;
  a64 *a64_end;

  va64 *va64_ptr;
  va64 *va64_end;

  a64v_state_tag tag;
  unsigned _unused;
} a64v_state;

void a64v_state_init(
  a64v_state *state,
  a64* a64_data, unsigned a64_count,
  va64* va64_data, unsigned va64_count);

bool a64v_emit_a64(a64v_state *, a64);
bool a64v_emit_va64(a64v_state *, va64);
