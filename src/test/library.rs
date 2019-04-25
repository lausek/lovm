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
    builder.decl("foo", foo);
    builder.decl("bar", bar);

    let module = builder.build().expect("building module failed");
}

#[test]
fn fib_function() {
    let fib = FunctionBuilder::new()
        .with_args(vec!["x"])
        .step(Operation::new(OperationType::Add).update().op("x").op(1))
        .debug()
        .build()
        .expect("building function failed");

    let module = ModuleBuilder::from_object(fib)
        .build()
        .expect("building module failed");
    println!("{:?}", module);

    fn debug(data: &mut vm::VmData) -> vm::VmResult {
        println!("locals: {:?}", data.stack.last().unwrap().locals);
        Ok(())
    }

    let mut vm = vm::Vm::new();
    vm.interrupts_mut()
        .set(vm::Interrupt::Debug as usize, &debug);
    vm.run(&module).expect("error in code");

    assert!(false);
}

fn gen_foo() -> BuildResult<Function> {
    // pseudocode:
    //      f(x, y):
    //          z = 1
    //          z += x
    //          z += y
    //          return z ; not implemented
    FunctionBuilder::new()
        .with_args(vec!["x", "y"])
        .step(Operation::new(OperationType::Ass).op("z").op(1))
        .step(Operation::new(OperationType::Add).update().op("z").op("x"))
        .step(Operation::new(OperationType::Add).update().op("z").op("y"))
        .build()
}
