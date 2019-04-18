use lovm::*;
use lovm_asm_lib::*;

use std::env;
use std::io::Read;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("no program specified");

    let mut vm = vm::Vm::new();

    let mut file = std::fs::File::open(&path).expect("cannot read file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("reading file failed");

    let unit = compiler::Compiler::new()
        .compile(src.as_ref(), path)
        .expect("compilation failed");

    let program = into_program(unit);

    println!("{}", program);

    vm.run(&program).unwrap();
}
