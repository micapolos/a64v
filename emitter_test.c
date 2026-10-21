#include <stdio.h>
#include <assert.h>

#include "a64.h"
#include "va64.h"
#include "emitter.h"

int main() {
  a64_emitter emitter;

  {
    a64_emitter_init(&emitter);
    assert(emitter.tag == a64_emitter_tag_init);
  }

  {
    a64_emitter_init(&emitter);
    a64_emitter_emit_a64(&emitter, A64_NOP);
    //assert(emitter.tag == a64_emitter_tag_error);
  }

  {
    a64_emitter_init(&emitter);
    a64_emitter_emit_va64(&emitter, VA64_STEP(0));
    //assert(emitter.tag == a64_emitter_tag_error);
  }

  return 0;
}
