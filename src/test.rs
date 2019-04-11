#!(cfg(test))

#[allow(dead_code)]
fn read_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).expect("cannot read file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("reading file failed");
    src
}

macro_rules! test_file {
    ($name:ident, $tester:tt) => {
        #[test]
        fn $name() {
            let path = format!("./asm/example/{}.loas", stringify!($name));
            let mut vm = lovm::vm::Vm::new();
            let src = read_file(path.as_str());
            let program = lovm_asm_lib::compiler::Compiler::new()
                .compile(src.as_ref())
                .expect("compilation failed");
            vm.run(&program).unwrap();
            $tester(vm);
        }
    };
}

test_file!(basic, (|_| {}));

test_file!(call, (|_| {}));

test_file!(fib, (|_| {}));

test_file!(mem, (|_| {}));

test_file!(mem2, (|_| {}));

test_file!(mem3, (|_| {}));

test_file!(ops, (|_| {}));
