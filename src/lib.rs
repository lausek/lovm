#![feature(type_alias_enum_variants)]
#![feature(const_vec_new)]

pub mod data;
#[macro_use]
pub mod gen;
pub mod test;
pub mod vm;

pub use data::*;

use std::borrow::Borrow;
use std::rc::Rc;

#[macro_export]
macro_rules! lovm_value {
    ($ty:ident, $val:expr) => {
        lovm::Value::$ty($val)
    };
}

// TODO: implement dict aswell
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
