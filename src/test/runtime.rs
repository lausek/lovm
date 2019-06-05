#![cfg(test)]

use super::*;

use crate::gen::*;

#[test]
fn allocation() {
    let mut func = FunctionBuilder::new();
    func.step(Operation::onew().end());
    func.step(Operation::odispose().end());
    func.step(Operation::onewarray().end());
    func.step(Operation::onewdict().end());
    func.step(Operation::odispose().end());
    func.debug();

    fn has_oref(data: &mut vm::VmData) -> vm::VmResult {
        assert!(*data.vstack.last().unwrap() == Value::Ref(2));
        Ok(())
    }

    run!(func.build().unwrap(), has_oref);
}

#[test]
fn new_dict() {
    let mut func = FunctionBuilder::new();
    func.step(Operation::onewdict().end());
    func.debug();

    fn has_oref(data: &mut vm::VmData) -> vm::VmResult {
        assert!(*data.vstack.last().unwrap() == Value::Ref(1));
        Ok(())
    }

    run!(func.build().unwrap(), has_oref);
}

#[test]
fn quirks() {
    let mut func = FunctionBuilder::new();

    // valid
    func.step(Operation::push().op(vec![1, 2, 3]).end());

    // invalid
    func.step(Operation::onewarray().op(vec![1, 2, 3]).end());

    run!(func.build().unwrap());

    assert!(false);
}
