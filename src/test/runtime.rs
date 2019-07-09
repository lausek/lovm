#![cfg(test)]
use super::*;

#[test]
fn allocation() {
    let func = func!({
        onew("object"),
        odispose(),
        onewarray(),
        onewdict(),
        odispose(),
        debug(),
    });

    fn has_oref(data: &mut VmData) -> VmResult {
        assert!(*data.vstack.last().unwrap() == value!(2; Ref));
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

    fn check_content(data: &mut VmData) -> VmResult {
        assert!(*data.vstack.last().unwrap() == Value::Ref(1));
        match &data.obj_pool.get(&1).expect("no object").inner {
            ObjectKind::Dict(object) => {
                let eobject = object! [
                    x => 10; I64,
                    y => 10; I64,
                    10; I64 => 11; I64,
                ];
                assert_eq!(object, &eobject);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    run!(func, check_content);
}
