use super::*;

pub type Macro = &'static dyn Fn(&mut Unit, Vec<Operand>) -> Result<(), Error>;
pub type MacroTable = HashMap<&'static str, Macro>;

pub fn default_macros() -> MacroTable {
    let mut macs = MacroTable::new();
    macs.insert("skip", &skip);
    macs
}

fn skip(unit: &mut Unit, args: Vec<Operand>) -> Result<(), Error> {
    let n = match &args[0] {
        Operand::Value(value) => usize::from(*value),
        _ => unreachable!(),
    };
    for _ in 0..n {
        unit.codeblock.push(Code::Value(Value::I(0)));
    }
    Ok(())
}
