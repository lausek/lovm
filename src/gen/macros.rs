use super::*;

#[macro_export]
macro_rules! unit {
    {$(
        // TODO: make `=> $func` optional and let $name = $func if missing
        $name:ident => $func:expr
    ),*} => {{
        let mut unit = UnitBuilder::new();
        $(
            unit.decl(stringify!($name), $func.into());
        )*
        unit.build().expect("building unit failed")
    }};
}

#[macro_export]
macro_rules! func {
    (
        $([$($param:ident),*] =>)?
        {
            $($op:expr
                $(=> { $($then:expr),* $(,)? })?
            ),* $(,)?
        }
    ) => {{
        let mut func = FunctionBuilder::new();
        $(
            let mut func = func.with_params::<&str>(vec![$(stringify!($param)),*]);
        )?
        $(
            func.step($op.end());
            $(
                func.branch_if(vec![$($then),*]);
            )?
        )*
        func.build().expect("building func failed")
    }};
}
