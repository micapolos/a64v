CC = clang
CFLAGS = -std=c23 -Wall -Wextra -MMD -MP
BUILD_DIR = built

TARGETS = \
	$(BUILD_DIR)/a64_test \
	$(BUILD_DIR)/va64_test \
	$(BUILD_DIR)/validator_test \
	$(BUILD_DIR)/a64v

all: $(TARGETS)

$(BUILD_DIR):
	mkdir -p $@

# Object file compilation rules with automatic dependency generation (-MMD -MP)
$(BUILD_DIR)/a64_test.o: a64_test.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/va64_test.o: va64_test.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/validator.o: validator.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/validator_test.o: validator_test.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/file.o: file.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/a64v.o: a64v.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Linking executables
$(BUILD_DIR)/a64_test: $(BUILD_DIR)/a64_test.o
	$(CC) $^ -o $@

$(BUILD_DIR)/va64_test: $(BUILD_DIR)/va64_test.o
	$(CC) $^ -o $@

$(BUILD_DIR)/validator_test: $(BUILD_DIR)/validator_test.o $(BUILD_DIR)/validator.o
	$(CC) $^ -o $@

$(BUILD_DIR)/a64v: $(BUILD_DIR)/a64v.o $(BUILD_DIR)/file.o $(BUILD_DIR)/validator.o
	$(CC) $^ -o $@

DEPS = $(wildcard $(BUILD_DIR)/*.d)
-include $(DEPS)

test: all
	./$(BUILD_DIR)/a64_test
	./$(BUILD_DIR)/va64_test
	./$(BUILD_DIR)/validator_test

run: test
	./$(BUILD_DIR)/a64v $(ARGS)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all test run clean
