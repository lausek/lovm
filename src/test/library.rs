#!(cfg(test))

use super::*;

use crate::gen::*;

#[test]
fn simple_function() {
    let func = gen_foo().expect("building function failed");
}

#[test]
fn simple_module() {
    let foo = gen_foo().expect("building `foo` failed");
    let bar = gen_foo().expect("building `bar` failed");

    let mut builder = ModuleBuilder::new();
    builder.decl("foo", foo).decl("bar", bar);

    let module = builder.build().expect("building module failed");
}

#[test]
fn fib_function() {
    let mut fib = FunctionBuilder::new().with_args(vec!["x"]);
    fib.step(Operation::cmp().var("x").op(0).end())
        .branch(
            Operation::jeq(),
            vec![Operation::push().var("x").end(), Operation::ret()],
        )
        .step(Operation::cmp().var("x").op(1).end())
        .branch(
            Operation::jeq(),
            vec![Operation::push().var("x").end(), Operation::ret()],
        )
        .step(Operation::sub().var("x").op(1).end())
        .step(Operation::call("fib"))
        .step(Operation::sub().var("x").op(2).end())
        .step(Operation::call("fib"))
        .step(Operation::add())
        .step(Operation::ret());
    let fib = fib.build().expect("building function failed");

    let mut main = FunctionBuilder::new();
    main.step(Operation::call("fib").op(8).end()).debug();
    let main = main.build().expect("building function failed");

    let mut module = ModuleBuilder::new();
    module.decl("fib", fib).decl("main", main);
    let module = module.build().expect("building module failed");

    fn debug(data: &mut vm::VmData) -> vm::VmResult {
        let frame = data.stack.last_mut().unwrap();
        println!("{:?}", frame);
        let result = data.vstack.pop().expect("no value");
        assert!(result == Value::I(21));
        Ok(())
    }

    let mut vm = vm::Vm::new();
    vm.interrupts_mut()
        .set(vm::Interrupt::Debug as usize, &debug);
    vm.run(&module).expect("error in code");
}

fn gen_foo() -> BuildResult<Function> {
    // pseudocode:
    //      f(x, y):
    //          z = 1
    //          z += x
    //          z += y
    //          return z ; not implemented
    let mut func = FunctionBuilder::new().with_args(vec!["x", "y"]);
    func.step(Operation::ass().op("z").op(1).end())
        .step(Operation::add().update().op("z").op("x").end())
        .step(Operation::add().update().op("z").op("y").end());
    func.build()
}
