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
        let dict = &mut data.obj_pool.get_mut(&1).expect("no object").borrow_mut();
        let dict = dict.as_indexable().expect("not indexable");
        assert_eq!(dict.getk(&Value::from("x")).unwrap(), &Value::I64(10));
        assert_eq!(dict.getk(&Value::from("y")).unwrap(), &Value::I64(10));
        assert_eq!(dict.getk(&Value::from(10)).unwrap(), &Value::I64(11));
        Ok(())
    }

    run!(func, check_content);
}

#[test]
fn object_call() {
    let func = func!({
        onewdict(),
        oset()
            // store 10 in key "x"
            .op("x").op(10)
            // store 10 in key "y"
            .op("y").op(10)
            // store 11 in key 10
            .op(10).op(11),
        ocall("len"),
        debug(),
    });

    fn check_content(data: &mut VmData) -> VmResult {
        // expect len for 3 items
        assert!(*data.vstack.last().unwrap() == Value::Ref(3));
        Ok(())
    }

    run!(func, check_content);
}
