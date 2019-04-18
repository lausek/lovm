use super::*;

pub type Macro = &'static dyn Fn(&mut Unit, Vec<Operand>) -> Result<(), Error>;
pub type MacroTable = HashMap<&'static str, Macro>;

pub fn default_macros() -> MacroTable {
    let mut macs = MacroTable::new();
    macs.insert("include", &include);
    macs.insert("skip", &skip);
    macs
}

fn include(_unit: &mut Unit, _args: Vec<Operand>) -> Result<(), Error> {
    // TODO: waiting for compiler
    /*
    use std::path::Path;
    if let Some(Operand::Str(path)) = args.get(0) {
        let parent = Path::new(&unit.path).parent().unwrap().display();
        let path = format!("{}/{}", parent, path);
        let mut program = crate::compile_file(&path)?;
        let link_off = unit.codeblock.len();
        unit.codeblock.extend(program.code());

        for (ident, off) in program.labels().iter() {
            let ident = Ident::new((0, 0, 0), ident.clone());
            match unit.labels.get(&ident) {
                Some(LabelOffset::Resolved(_)) => panic!("label already exists in super program"),
                Some(LabelOffset::Unresolved(positions)) => {
                    for (_, pos) in positions.iter().rev() {
                        *unit.codeblock.get_mut(*pos).unwrap() = mkref(link_off + *off);
                    }
                }
                _ => {
                    unit.labels.insert(ident, LabelOffset::Resolved(link_off + *off));
                }
            }
        }
    }
    */
    Ok(())
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
