use super::*;

pub fn default_macros() -> HashMap<&'static str, Macro> {
    let mut macs = HashMap::new();
    macs.insert("skip", Box::new(skip));
    macs
}

fn skip(unit: &mut Unit) -> Result<(), Error> {
    Ok(())
}
