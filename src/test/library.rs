#![cfg(test)]

use super::*;

use crate::gen::*;

#[test]
fn simple_function() {
    let _func = gen_foo();
}

#[test]
fn simple_module() {
    let foo = gen_foo();
    let bar = gen_foo();

    let _unit = unit! {
        foo => foo,
        bar => bar
    };
}

#[test]
fn fib_function() {
    let mut fib = FunctionBuilder::new().with_params(vec!["x"]);
    let ret_x = vec![Operation::ret().var("x").end()];
    fib.step(Operation::cmp_eq().var("x").op(0).end())
        .branch_if(ret_x.clone())
        .step(Operation::cmp_eq().var("x").op(1).end())
        .branch_if(ret_x.clone())
        .step(
            Operation::add()
                .op(Operation::call("fib")
                    .op(Operation::sub().var("x").op(1).end())
                    .end())
                .op(Operation::call("fib")
                    .op(Operation::sub().var("x").op(2).end())
                    .end())
                .end(),
        )
        .step(Operation::ret());
    println!("{}", fib);
    let fib = fib.build().expect("building function failed");

    let mut main = func!({
        Operation::call("fib").op(8).end(),
        Operation::debug(),
    });

    let unit = unit! {
        main => main,
        fib => fib
    };

    fn debug(data: &mut vm::VmData) -> vm::VmResult {
        let frame = data.stack.last_mut().unwrap();
        println!("{:?}", frame);
        let result = data.vstack.pop().expect("no value");
        assert!(result == Value::I64(21));
        Ok(())
    }

    let mut vm = vm::Vm::new();
    vm.interrupts_mut()
        .set(vm::Interrupt::Debug as usize, &debug);
    vm.run(&unit).expect("error in code");
}

#[allow(dead_code)]
fn gen_foo() -> CodeObject {
    // pseudocode:
    //      f(x, y):
    //          z = 1
    //          z += x
    //          z += y
    //          return z ; not implemented
    func!([x] => {
        Operation::ass().var("z").op(1).end(),
        Operation::add().var("z").op("x").end(),
        Operation::add().var("z").op("y").end()
    })
}
