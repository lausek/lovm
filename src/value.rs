#[derive(Clone, Copy, Debug)]
pub enum Value {
    I(i8),
    U(u8),
    I64(i64),
    U64(u64),
    T(bool),
    // TODO: add reg and str?
}
