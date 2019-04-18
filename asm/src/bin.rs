extern crate lovm_asm_lib;

use lovm_asm_lib::*;

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let mut args = env::args().skip(1);
    let in_file_path = args.next().expect("no program specified");
    let out_file_path = args.next().unwrap_or("a.out".to_string());

    match compile_file(&in_file_path) {
        Ok(unit) => {
            let program = into_program(unit);
            if let Ok(mut out_file) = fs::File::create(out_file_path) {
                let bytes = program.serialize().unwrap();
                out_file.write_all(bytes.as_slice()).unwrap();
            } else {
                eprintln!("cannot create file");
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
