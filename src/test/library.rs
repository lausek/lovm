#!(cfg(test))

use super::*;

use crate::gen::*;

#[test]
fn simple_function() {
    let _func = gen_foo().expect("building function failed");
}

#[test]
fn simple_module() {
    let foo = gen_foo().expect("building `foo` failed");
    let bar = gen_foo().expect("building `bar` failed");

    let mut builder = ModuleBuilder::new();
    builder.decl("foo", foo.into()).decl("bar", bar.into());

    let _module = builder.build().expect("building module failed");
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

    let mut main = FunctionBuilder::new();
    main.step(Operation::call("fib").op(8).end()).debug();
    let main = main.build().expect("building function failed");

    let mut module = ModuleBuilder::new();
    module.decl("fib", fib.into()).decl("main", main.into());
    let module = module.build().expect("building module failed");

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
    vm.run(&module).expect("error in code");
}

#[allow(dead_code)]
fn gen_foo() -> BuildResult<CodeObject> {
    // pseudocode:
    //      f(x, y):
    //          z = 1
    //          z += x
    //          z += y
    //          return z ; not implemented
    let mut func = FunctionBuilder::new().with_params(vec!["x", "y"]);
    func.step(Operation::ass().var("z").op(1).end())
        .step(Operation::add().var("z").op("x").end())
        .step(Operation::add().var("z").op("y").end());
    func.build()
}
