extern crate lovm_asm_lib;

use lovm_asm_lib::*;

use std::env;
use std::io::Read;

fn main() {
    let mut args = env::args().skip(1);
    let program = args.next().expect("no program specified");

    if let Ok(mut file) = std::fs::File::open(program) {
        let mut src = String::new();
        file.read_to_string(&mut src)
            .expect("cannot read program file");

        let result = compiler::Compiler::new().compile(src.as_ref());
        println!("{:?}", result);
    } else {
        panic!("cannot open program file");
    }
}
