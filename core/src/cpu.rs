use crate::{Registers, Instruction, ArithmeticTarget, Memory};

#[derive(Default)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory
}
impl Cpu {
    pub fn step(&mut self) {
      let mut instruction_byte = self.memory[self.registers.pc];
  
      let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
        self.execute(instruction)
      } else {
        panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
      };
  
      self.pc = next_pc;
    }
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(arithmetic_target) => match arithmetic_target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                _ => todo!()
            }
        }
    }
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}