use lovm::*;

use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let program = args.next().expect("no program specified");
}
