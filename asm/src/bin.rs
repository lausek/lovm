extern crate lovm_asm_lib;

use lovm_asm_lib::*;

fn main() {
    let result = compiler::compile("mov A, #1\nadd A, #2");
    println!("{:?}", result);
}
