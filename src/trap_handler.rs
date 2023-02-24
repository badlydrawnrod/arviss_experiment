// Trap handling is a trait.
pub trait TrapHandler {
    type Item;

    fn handle_trap(&self) -> Self::Item;
    fn handle_ecall(&self) -> Self::Item;
    fn handle_ebreak(&self) -> Self::Item;
}

pub struct BasicTrapHandler {
    // TODO...
}

impl BasicTrapHandler {
    pub fn new() -> Self {
        BasicTrapHandler {  }
    }
}

impl TrapHandler for BasicTrapHandler {
    type Item = ();

    fn handle_trap(&self) {
        println!("TRAP!");
    }

    fn handle_ecall(&self) {
        println!("ECALL!");
    }

    fn handle_ebreak(&self) {
        println!("EBREAK!");
    }
}

