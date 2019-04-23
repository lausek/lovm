#!(cfg(test))

use super::*;

use crate::gen::*;

#[test]
fn simple_function() {
    let func = gen_foo().expect("building function failed");
    println!("{:?}", func);

    assert!(false);
}

#[test]
fn simple_module() {
    let foo = gen_foo().expect("building `foo` failed");
    let bar = gen_foo().expect("building `bar` failed");

    let mut builder = ModuleBuilder::new();
    builder.decl("foo", foo);
    builder.decl("bar", bar);

    let module = builder.build().expect("building module failed");
    println!("{:?}", module);

    assert!(false);
}

fn gen_foo() -> BuildResult<Function> {
    // pseudocode:
    //      f(x, y):
    //          z = x + y
    //          return z
    let mut builder = FunctionBuilder::new().with_args(vec!["x", "y"]);
    builder.build()
}
