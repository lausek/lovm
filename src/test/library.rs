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
        .step(Operation::new(OperationType::Cmp).var("x").op(0))
        .branch(
            Operation::new(OperationType::Jeq),
            vec![
                Operation::new(OperationType::Add).var("x").op(0), // TODO: this is a hack for pushing x
                Operation::new(OperationType::Ret),
            ],
        )
        .step(Operation::new(OperationType::Cmp).var("x").op(1))
        .branch(
            Operation::new(OperationType::Jeq),
            vec![
                Operation::new(OperationType::Add).var("x").op(0), // TODO: this is a hack for pushing x
                Operation::new(OperationType::Ret),
            ],
        )
        .step(Operation::new(OperationType::Sub).var("x").op(1))
        .step(Operation::new(OperationType::Call).op("fib"))
        .step(Operation::new(OperationType::Sub).var("x").op(2))
        .step(Operation::new(OperationType::Call).op("fib"))
        .step(Operation::new(OperationType::Add))
        .step(Operation::new(OperationType::Ret))
        .build()
        .expect("building function failed");

    let main = FunctionBuilder::new()
        .step(Operation::new(OperationType::Call).op("fib").op(8))
        .debug()
        .build()
        .expect("building function failed");

    let mut module = ModuleBuilder::new();
    module.decl("fib", fib);
    module.decl("main", main);
    let module = module.build().expect("building module failed");

    fn debug(data: &mut vm::VmData) -> vm::VmResult {
        let frame = data.stack.last_mut().unwrap();
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
    FunctionBuilder::new()
        .with_args(vec!["x", "y"])
        .step(Operation::new(OperationType::Ass).op("z").op(1))
        .step(Operation::new(OperationType::Add).update().op("z").op("x"))
        .step(Operation::new(OperationType::Add).update().op("z").op("y"))
        .build()
}
