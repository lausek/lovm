#!(cfg(test))

use super::*;

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
            let mut compiler = lovm_asm_lib::compiler::Compiler::new();
            compiler
                .compile_path(src.as_ref(), path)
                .expect("compilation failed");
            let unit = compiler.finish().expect("linking failed");
            let program = lovm_asm_lib::into_program(unit);

            vm.interrupts_mut()
                .set(Interrupt::Dbg as usize, Some(&$tester));

            vm.run(&program).unwrap();
        }
    };
}

test_file!(basic, (|_| { Ok(()) }));

test_file!(call, (|_| { Ok(()) }));

test_file!(
    fib,
    (|vm| {
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

test_file!(mem, (|_vm| { Ok(()) }));

test_file!(
    mem2,
    (|vm| {
        use lovm::code::Code;
        use lovm::value::Value;

        //assert_eq!(vm.memory[200], Code::Value(Value::I64(666)));
        //assert_eq!(vm.memory[201], Code::Value(Value::I64(667)));

        Ok(())
    })
);

test_file!(mem3, (|_| { Ok(()) }));

test_file!(ops, (|_| { Ok(()) }));

test_file!(
    cast,
    (|vm| {
        use lovm::value::Value;

        let reg = vm.stack.last().unwrap();

        // TODO: add type checks here
        assert_eq!(reg.a, Value::Ref(20));
        assert_eq!(reg.b, Value::F64(10.));
        assert_eq!(reg.c, Value::I(30));
        assert_eq!(reg.d, Value::T(false));

        Ok(())
    })
);

test_file!(str, (|_| { Ok(()) }));
