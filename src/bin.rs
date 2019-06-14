use lovm::*;

use std::env;
use std::io::Read;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("no program specified");

    let mut vm = vm::Vm::new();

    let mut file = std::fs::File::open(&path).expect("cannot read file");
    let mut src = vec![];
    file.read_to_end(&mut src).expect("reading file failed");

    let unit = Unit::deserialize(src.as_ref()).expect("deserialize failed");

    vm.run(&unit).unwrap();
}
