#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "a64.h"
#include "va64.h"
#include "file.h"
#include "validator.h"

static a64_validator validator;

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Usage: %s <file.a64> <file.va64> arg ...\n", argv[0]);
    return 1;
  }

  a64v_with_file_data(argv[1], ^(const void *a64_data, size_t a64_size) {
    a64v_with_file_data(argv[2], ^(const void *va64_data, size_t va64_size) {
      a64_validator_init(&validator);
      a64_validator_emit(&validator,
        (const a64*)a64_data, a64_size >> 2,
        (const va64*)va64_data, va64_size >> 2);
    });
  });

  return 0;
}
