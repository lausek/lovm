extern crate lovm_asm_lib;

use lovm_asm_lib::*;

use std::env;
use std::fs;
use std::io::{Read, Write};

fn main() {
    let mut args = env::args().skip(1);
    let in_file_path = args.next().expect("no program specified");
    let out_file_path = args.next().unwrap_or("a.out".to_string());

    if let Ok(mut in_file) = fs::File::open(in_file_path) {
        let mut src = String::new();
        in_file
            .read_to_string(&mut src)
            .expect("cannot read program file");

        let result = compiler::Compiler::new().compile(src.as_ref());
        match result {
            Ok(program) => {
                println!("{:?}", program);
                if let Ok(mut out_file) = fs::File::create(out_file_path) {
                    let bytes = program.serialize().unwrap();
                    out_file.write_all(bytes.as_slice()).unwrap();
                } else {
                    panic!("cannot create file");
                }
            }
            _ => panic!("{:?}", result),
        }
    } else {
        panic!("cannot open program file");
    }
}
