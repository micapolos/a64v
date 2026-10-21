#pragma once

typedef void (^a64v_file_data_callback_t)(const void *data, size_t size);

void a64v_with_file_data(
  const char *filename,
  __attribute__((noescape)) a64v_file_data_callback_t);
