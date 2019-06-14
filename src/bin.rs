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

    // TODO: add `deserialize_from` to deserialize from a file
    //let unit = Unit::deserialize(src).expect("deserialize failed");
    //let module: Unit = bincode::deserialize(src.as_ref()).expect("deserialize failed");

    //vm.run(&unit).unwrap();
}
