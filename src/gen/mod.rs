pub mod func;
pub mod op;
pub mod seq;

pub use func::*;
pub use op::*;
pub use seq::*;

use super::*;

// TODO: export functionality for generating lovm programs

// ---- example
// pseudocode:
//      f(x, y):
//          z = x + y
//          return z
// rust
//      gen::Function::new()
//          .with_args(vec!["x", "y"])      // TODO: is it `args` or `params` here? there was a difference...
//          .step(gen::Op::Add, "x", "y")
//          .store("z")
//          .end()
//          .build()
//
// ---- explanation
