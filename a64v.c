#include <stdio.h>
#include <stdlib.h>

#include "a64.h"
#include "va64.h"
#include "a64v.h"

void a64v_state_init(
    a64v_state *state,
    const a64* a64_data, unsigned a64_count,
    const va64* va64_data, unsigned va64_count) {
  state->a64_ptr = (a64*) a64_data;
  state->a64_end = (a64*) a64_data + a64_count;
  state->va64_ptr = (va64*) va64_data;
  state->va64_end = (va64*) va64_data + va64_count;
  state->tag = a64v_state_tag_start;
}
