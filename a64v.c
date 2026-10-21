#include <stdio.h>
#include <stdlib.h>

#include "a64.h"
#include "va64.h"
#include "file.h"
#include "verifier.h"

void a64_verify(
    const a64 *a64_ops, size_t a64_size,
    const va64 *va64_ops, size_t va64_size) {
  a64_verifier verifier;

  printf("Verify start\n");
  a64_verifier_init(&verifier);

  a64* a64_ptr = (a64*)a64_ops;
  a64* a64_end = a64_ptr + (a64_size >> 2);

  va64* va64_ptr = (va64*)va64_ops;
  va64* va64_end = a64_ptr + (va64_size >> 2);

  unsigned a64_offset = 0;
  while (a64_ptr != a64_end) {
    while (a64_offset == 0 && va64_ptr != va64_end) {
      unsigned va64_op = *va64_ptr;
      if (va64_get_tag(va64_op) == VA64_TAG_STEP) {
        printf("va64: %p", va64_ptr);
        a64_offset += va64_get_step_offset(va64_op);
        va64_ptr++;
      } else {
        a64_verifier_emit_va64(&verifier, va64_op);
        continue;
      }
    }

    if (a64_offset == 0) {
      a64_offset--;
      a64 a64_op = *a64_ptr;
      a64_verifier_emit_a64(&verifier, a64_op);
      printf("a64: %p", a64_ptr);
      a64_ptr++;
    }
  }
  printf("Verify end\n");
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Usage: %s <file.a64> <file.va64> arg ...\n", argv[0]);
    return 1;
  }

  a64v_with_file_data(argv[1], ^(const void *a64_data, size_t a64_size) {
    a64v_with_file_data(argv[2], ^(const void *va64_data, size_t va64_size) {
      a64_verify(
        (const a64*)a64_data, a64_size >> 2,
        (const va64*)va64_data, va64_size >> 2);
    });
  });

  return 0;
}
