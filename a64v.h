typedef enum {
  va64_tag_error = 1,
} va64_tag_t;

typedef struct va64_state {
  va64_tag_t tag;
} va64_state_t;
