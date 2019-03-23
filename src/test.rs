#!(cfg(test))

use crate::vm::Vm;

#[test]
fn test() {
    let mut vm = Vm::new();
    let code = code! {
        Inc, C;
        Add, A, #U(1);
        Cmp, A, #U(10);
        Jne, #U(0);
        Pusha;
        Mov, B, #U(0b1111_0000);
        Xor, B, #U(0b0000_1111);
        Popa;
        Cmp, C, #U(100);
        Jeq, #U(0);
    };

    println!("bytecode: {:?}", code);

    vm.run(&code).unwrap();

    assert!(false);
}
