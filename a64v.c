#include <stdio.h>
#include <stdlib.h>

#include "a64.h"
#include "va64.h"
#include "a64v.h"

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Usage: %s <file.a64> <file.va64> arg ...\n", argv[0]);
    return 1;
  }

  FILE *a64file = fopen(argv[1], "rb");
  if (a64file == NULL) {
    printf("Could not open %s", argv[1]);
    return 1;
  }

  FILE *va64file = fopen(argv[2], "rb");
  if (va64file == NULL) {
    printf("Could not open %s", argv[2]);
    fclose(a64file);
    return 1;
  }

  fclose(va64file);
  fclose(a64file);

  return 0;
}
