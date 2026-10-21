CC = clang
CFLAGS = -std=c23 -Wall -Wextra -Wpadded -Werror=padded
BUILD_DIR = built

TARGETS = \
  $(BUILD_DIR)/a64_test \
  $(BUILD_DIR)/va64_test \
  $(BUILD_DIR)/validator_test \
  $(BUILD_DIR)/a64v

all: $(TARGETS)

$(BUILD_DIR):
	mkdir -p $@

$(BUILD_DIR)/a64_test: a64_test.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) $< -o $@

$(BUILD_DIR)/va64_test: va64_test.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) $< -o $@

$(BUILD_DIR)/validator_test: validator.c validator_test.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ -o $@

$(BUILD_DIR)/a64v: file.c validator.c a64v.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) $^ -o $@

test: all
	./$(BUILD_DIR)/a64_test
	./$(BUILD_DIR)/va64_test
	./$(BUILD_DIR)/validator_test

run: test
	./$(BUILD_DIR)/a64v $(ARGS)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all test run clean
