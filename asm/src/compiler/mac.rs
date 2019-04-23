use super::*;

pub type Macro = &'static dyn Fn(&mut Unit, Vec<Operand>) -> Result<(), Error>;
pub type MacroTable = HashMap<&'static str, Macro>;

pub fn default_macros() -> MacroTable {
    let mut macs = MacroTable::new();
    macs.insert("export", &export);
    macs.insert("include", &include);
    macs
}

fn export(unit: &mut Unit, args: Vec<Operand>) -> Result<(), Error> {
    if let Some(Operand::Ident(ident)) = args.get(0) {
        match unit.labels.get_mut(&ident) {
            Some(label) => label.public = true,
            _ => {
                let mut label = Label::new();
                label.public = true;
                unit.labels.insert(ident.clone(), label);
            }
        }
        Ok(())
    } else {
        unimplemented!()
    }
}

fn include(unit: &mut Unit, args: Vec<Operand>) -> Result<(), Error> {
    use std::path::Path;
    if let Some(Operand::Str(path)) = args.get(0) {
        let parent_path = unit.path.as_ref().unwrap().clone();
        let parent = Path::new(&parent_path).parent().unwrap().display();
        let path = format!("{}/{}", parent, path);
        let extern_unit = crate::compile_file(&path)?;
        unit.sub_units.push(extern_unit);
    } else {
        // TODO: raise `expected_got` here
        unimplemented!();
    }
    Ok(())
}
