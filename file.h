#pragma once

#include "data.h"

void a64v_with_file_data(const char *filename, __attribute__((noescape)) a64v_data_callback_t callback);
