#![cfg(test)]

use super::*;

#[allow(unused_imports)]
use crate::gen::*;

#[test]
fn allocation() {
    let func = func!({
        onew(),
        odispose(),
        onewarray(),
        onewdict(),
        odispose(),
        debug(),
    });

    fn has_oref(data: &mut vm::VmData) -> vm::VmResult {
        assert!(*data.vstack.last().unwrap() == Value::Ref(2));
        Ok(())
    }

    run!(func, has_oref);
}

#[test]
fn new_dict() {
    let func = func!({
        onewdict(),
        oset()
        // store 10 in key "x"
        .op("x").op(10)
        // store 10 in key "y"
        .op("y").op(10)
        // store 11 in key 10
        .op(10).op(11),
        debug(),
    });

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

    run!(func, check_content);
}
