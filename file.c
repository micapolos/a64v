#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

#include "file.h"

void a64v_with_file_data(const char *filename, a64v_data_callback_t callback) {
  int fd = open(filename, O_RDONLY);
  if (fd == -1) {
    perror("Failed to open file");
    return;
  }

  struct stat sb;
  if (fstat(fd, &sb) == -1) {
    perror("Failed to get file size");
    close(fd);
    return;
  }
  size_t size = (size_t)sb.st_size;

  if (size == 0) {
    close(fd);
    return;
  }

  void *data = mmap(NULL, size, PROT_READ, MAP_PRIVATE, fd, 0);
  if (data == MAP_FAILED) {
    perror("Failed to map file");
    close(fd);
    return;
  }

  callback(data, size);

  munmap(data, size);

  close(fd);
}
