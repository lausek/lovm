use super::*;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Interrupt {
    Dbg = 10,
    Put = 20,
}

pub type InterruptHandler = &'static dyn Fn(&mut VmData) -> VmResult;

pub struct Interrupts([Option<InterruptHandler>; 256]);

impl Interrupts {
    pub fn new() -> Self {
        Self([None; 256])
    }

    pub fn get(&self, idx: usize) -> Option<&InterruptHandler> {
        self.0[idx].as_ref()
    }

    pub fn set(&mut self, idx: usize, ir: Option<InterruptHandler>) {
        *self.0.get_mut(idx).unwrap() = ir;
    }
}

impl std::default::Default for Interrupts {
    fn default() -> Self {
        let mut ints = Interrupts::new();
        ints.set(Interrupt::Put as usize, Some(&put));
        ints
    }
}

fn put(data: &mut VmData) -> VmResult {
    let v = data.vstack.last().expect("no operand");
    let b = usize::from(*v) as u8;
    print!("{}", b as char);
    Ok(())
}
