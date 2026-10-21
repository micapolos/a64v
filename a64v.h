typedef enum : unsigned {
  a64v_state_tag_start = 0x0,
  a64v_state_tag_fn = 0x1,
  a64v_state_tag_error = 0xff,
} a64v_state_tag;

typedef struct {
  a64v_state_tag tag;
} a64v_state;

bool a64v_emit_a64(a64v_state *, a64);
bool a64v_emit_va64(a64v_state *, va64);
