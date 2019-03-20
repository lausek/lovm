#!(cfg(test))

use crate::vm::Vm;

#[test]
fn test() {
    let mut vm = Vm::new();
    let code = code! {
        Store, A, #U(1);
        Store, B, A;
        Add, B, #U(1);
        Store, C, B;
        Cmp, C, B;
        Cmp, D, #I(1);
    };

    vm.run(&code).unwrap();

    assert!(false);
}
