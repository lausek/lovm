#!(cfg(test))

use crate::vm::Vm;

#[test]
fn test() {
    let mut vm = Vm::new();
    let code = code! {
        Null, Null, Null
    };

    vm.run(&code).unwrap();

    assert!(false);
}
