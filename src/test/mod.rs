#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::gen::*;

pub mod library;
pub mod perf;
pub mod runtime;

#[macro_export]
macro_rules! run {
    ($func:expr $(, $dbg:expr)?) => {
        use crate::gen::*;
        let mut module = UnitBuilder::new();
        module.decl("main", $func.into());
        let module = module.build().expect("building module failed");

        let mut vm = vm::Vm::new();

        $(
            vm.interrupts_mut().set(vm::Interrupt::Debug as usize, &$dbg);
        )?

        vm.run(&module).expect("error in code");
    };
}
