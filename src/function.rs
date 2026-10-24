pub struct FunctionId(pub u16)
pub struct FunctionOffset(pub u32)

pub struct Function {
  pub id: FunctionId,
  pub offset: FunctionOffset,
}
