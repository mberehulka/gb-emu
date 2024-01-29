mod registers;    pub use registers::*;
mod instruction;  pub use instruction::*;
mod cpu;          pub use cpu::*;
mod memory;       pub use memory::*;

#[derive(Default)]
pub struct GameBoy {
    pub cpu: Cpu
}