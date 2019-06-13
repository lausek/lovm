#![feature(type_alias_enum_variants)]

#[macro_use]
pub mod code;
pub mod gen;
pub mod test;
pub mod value;
pub mod vm;

pub use code::*;
pub use value::*;

#[macro_export]
macro_rules! lovm_value {
    ($ty:ident, $val:expr) => {
        lovm::Value::$ty($val)
    };
}

#[macro_export]
macro_rules! lovm_object {
    () => {};
    [ $($val:pat),* ] => {{
        let array = lovm::vm::object::Array::new();
        $()*
        array
    }};
    [ $($key:ident => $val:pat),* ] => {};
}
