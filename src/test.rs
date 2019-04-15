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
            use lovm::vm::interrupt::Interrupt;
            let path = format!("./asm/example/{}.loas", stringify!($name));
            let mut vm = lovm::vm::Vm::new();
            let src = read_file(path.as_str());
            let unit = lovm_asm_lib::compiler::Compiler::new()
                .compile(src.as_ref())
                .expect("compilation failed");
            let program = lovm_asm_lib::into_program(unit);

            vm.interrupts_mut()
                .set(Interrupt::Dbg as usize, Some($tester));

            vm.run(&program).unwrap();
        }
    };
}

test_file!(basic, (|_| { Ok(()) }));

test_file!(call, (|_| { Ok(()) }));

test_file!(
    fib,
    (|vm| {
        use lovm::code::Code;
        use lovm::value::Value;

        let result3 = vm.vstack.pop().expect("no result3");
        let result2 = vm.vstack.pop().expect("no result2");
        let result1 = vm.vstack.pop().expect("no result1");

        assert_eq!(result3, Value::I64(21));
        assert_eq!(result2, Value::I64(1));
        assert_eq!(result1, Value::I64(0));

        Ok(())
    })
);

test_file!(
    mem,
    (|vm| {
        use lovm::code::Code;
        use lovm::value::Value;

        Ok(())
    })
);

test_file!(
    mem2,
    (|vm| {
        use lovm::code::Code;
        use lovm::value::Value;

        assert_eq!(vm.memory[200], Code::Value(Value::I64(666)));
        assert_eq!(vm.memory[201], Code::Value(Value::I64(667)));

        Ok(())
    })
);

test_file!(mem3, (|_| { Ok(()) }));

test_file!(ops, (|_| { Ok(()) }));
