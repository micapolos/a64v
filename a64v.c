#include <stdio.h>
#include <stdlib.h>

#include "a64.h"
#include "va64.h"
#include "a64_verifier.h"
#include "data.h"
#include "file.h"

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Usage: %s <file.a64> <file.va64> arg ...\n", argv[0]);
    return 1;
  }

  a64v_with_file_data(argv[1], ^(const void *a64_data, size_t a64_size) {
    a64v_with_file_data(argv[2], ^(const void *va64_data, size_t va64_size) {
      a64_verifier verifier;
      a64_verifier_init(&verifier);
      printf("%p, %zu, %p, %zu", a64_data, a64_size, va64_data, va64_size);
    });
  });

  return 0;
}
