#![feature(type_alias_enum_variants)]
#![feature(const_vec_new)]
#![feature(trivial_bounds)]

pub mod data;
#[macro_use]
pub mod gen;
pub mod test;
pub mod vm;

pub use data::*;
pub use vm::*;

use std::borrow::Borrow;
use std::rc::Rc;

#[macro_export]
macro_rules! value {
    ($val:ident) => { value!(stringify!($val).to_string(); Str)};
    ($val:expr; $ty:ident) => {{
        use crate::*;
        Value::$ty($val)
    }};
}

#[macro_export]
macro_rules! object {
    () => {};
    [ $($val:expr),* $(,)? ] => {{
        use crate::*;
        let mut array = lovm::vm::object::Array::new();
        {
            let array = array.inner_mut();
            $(
                array.push($val);
            )*
        }
        array
    }};
    [ $($key:tt $(; $kty:ident)? => $val:expr; $ty:ident),* $(,)? ] => {{
        use crate::*;
        let mut dict = vm::object::Dict::new();
        {
            let dict = dict.inner_mut();
            $(
                dict.insert(value!($key $(; $kty)?), value!($val; $ty));
            )*
        }
        dict
    }};
}
