use lovm::*;
use lovm_asm_lib::*;

use std::env;
use std::io::Read;

fn main() {
    let mut args = env::args().skip(1);
    let program = args.next().expect("no program specified");

    let mut vm = vm::Vm::new();

    let mut file = std::fs::File::open(program).expect("cannot read file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("reading file failed");

    let program = compiler::Compiler::new()
        .compile(src.as_ref())
        .expect("compilation failed");

    vm.run(&program).unwrap();
}
