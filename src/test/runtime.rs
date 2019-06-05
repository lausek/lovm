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
    func.step(Operation::onewdict());
    let dict = Operation::oset()
        // store 10 in key "x"
        .op("x")
        .op(10)
        // store 10 in key "y"
        .op("y")
        .op(10)
        // store 11 in key 10
        .op(10)
        .op(11)
        .end();
    func.step(dict);
    func.debug();

    fn check_content(data: &mut vm::VmData) -> vm::VmResult {
        use crate::vm::object::*;

        assert!(*data.vstack.last().unwrap() == Value::Ref(1));
        match &data.obj_pool.get(&1).expect("no object").inner {
            ObjectKind::Dict(object) => {
                println!("{:?}", object);
                assert_eq!(
                    *object.getk(&Value::Str("x".to_string())).unwrap(),
                    Value::I64(10)
                );
                assert_eq!(
                    *object.getk(&Value::Str("y".to_string())).unwrap(),
                    Value::I64(10)
                );
                assert_eq!(*object.getk(&Value::I64(10)).unwrap(), Value::I64(11));
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    run!(func.build().unwrap(), check_content);
}

#[test]
fn quirks() {
    let mut func = FunctionBuilder::new();

    // valid
    func.step(Operation::push().op(vec![1, 2, 3]).end());

    // invalid
    func.step(Operation::onewarray().op(vec![1, 2, 3]).end());

    run!(func.build().unwrap());
}
