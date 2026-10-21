typedef enum : uint8_t {
  a64v_state_tag_start = 0x0,
  a64v_state_tag_fn = 0x1,
  a64v_state_tag_error = 0xff,
} a64v_state_tag_t;

typedef struct {
  a64v_state_tag_t tag;
} a64v_state_t;

bool a64v_emit_a64(a64v_state_t *, a64_t);
bool a64v_emit_va64(a64v_state_t *, va64_t);
