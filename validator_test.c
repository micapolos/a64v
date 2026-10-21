#include <stdio.h>
#include <assert.h>

#include "a64.h"
#include "va64.h"
#include "validator.h"

int main() {
  a64_validator validator;

  {
    a64_validator_init(&validator);
    assert(validator.tag == a64_validator_tag_init);
  }

  {
    a64_validator_init(&validator);
    a64_validator_emit_a64(&validator, A64_NOP);
    assert(validator.tag == a64_validator_tag_error);
  }

  {
    a64_validator_init(&validator);
    a64_validator_emit_va64(&validator, VA64_STEP(0));
    assert(validator.tag == a64_validator_tag_error);
  }

  return 0;
}
