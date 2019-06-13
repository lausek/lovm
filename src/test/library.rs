#![cfg(test)]
use super::*;

#[test]
fn simple_function() {
    let _func = gen_foo();
}

#[test]
fn simple_module() {
    let _unit = unit! {
        foo => gen_foo(),
        bar => gen_foo(),
    };
}

#[test]
fn fib_function() {
    let ret_x = ret().var("x").end();
    let fib = func!([x] => {
        cmp_eq().var("x").op(0) => { ret_x.clone() },
        cmp_eq().var("x").op(1) => { ret_x.clone() },
        add()
            .op(call("fib").op(sub().var("x").op(1).end()).end())
            .op(call("fib").op(sub().var("x").op(2).end()).end()),
        ret()
    });

    let main = func!({
        call("fib").op(8),
        Operation::debug(),
    });

    let unit = unit! {
        main,
        fib,
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
        ass().var("z").op(1),
        add().var("z").op("x"),
        add().var("z").op("y"),
    })
}
