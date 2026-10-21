#include <stdio.h>
#include <stdlib.h>

#include "a64.h"
#include "va64.h"
#include "a64v.h"
#include "data.h"
#include "file.h"

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Usage: %s <file.a64> <file.va64> arg ...\n", argv[0]);
    return 1;
  }

  a64v_with_file_data(argv[1], ^(const void *a64_data, size_t a64_size) {
    a64v_with_file_data(argv[2], ^(const void *va64_data, size_t va64_size) {
      a64v_state state;
      a64v_state_init(
        &state,
        (a64*) a64_data, a64_size >> 2,
        (a64*) va64_data, va64_size >> 2);
    });
  });

  return 0;
}
