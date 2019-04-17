use super::*;

pub type Macro = &'static dyn Fn(&mut Unit) -> Result<(), Error>;
pub type MacroTable = HashMap<&'static str, Macro>;

pub fn default_macros() -> MacroTable {
    let mut macs = MacroTable::new();
    macs.insert("skip", &skip);
    macs
}

fn skip(_unit: &mut Unit) -> Result<(), Error> {
    Ok(())
}
