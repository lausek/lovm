#[allow(unused_imports)]
use super::*;

#[macro_export]
macro_rules! unit {
    (0 $unit:expr, $name:expr) => {
        $unit.decl(stringify!($name), $name);
    };
    (0 $unit:expr, $name:expr, $func:expr) => {
        $unit.decl(stringify!($name), $func.into());
    };
    {
        $($name:ident $(=> $func:expr)?),* $(,)?
    } => {{
        let mut unit = UnitBuilder::new();
        $(
            unit!(0 unit, $name $(, $func)?);
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
        #[allow(unused_mut)]
        let mut func = CodeBuilder::new();
        $(
            let mut func = func.with_params::<&str>(vec![$(stringify!($param)),*]);
        )?
        $(
            func.step($op.end());
            $(
                func.branch_if(vec![$($then.end()),*]);
            )?
        )*
        func.build(true).expect("building func failed")
    }};
}
