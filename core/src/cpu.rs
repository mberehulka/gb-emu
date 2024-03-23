use crate::{Memory, Registers};

#[derive(Default)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory
}
impl Cpu {
    pub fn step(&mut self) {
      let mut instruction = self.memory[self.registers.pc];
      let prefixed = instruction == 0xCB;
      if prefixed {
        instruction = self.memory[self.registers.pc + 1]
      }
      self.registers.pc = self.execute(prefixed, instruction);
    }

    fn execute(&mut self, prefixed: bool, instruction: u8) -> u16 {
        match (prefixed, instruction) {
            (false, 0x80) => self.add_a(self.registers.b),
            (false, 0x81) => self.add_a(self.registers.c),
            (false, 0x82) => self.add_a(self.registers.d),
            (false, 0x83) => self.add_a(self.registers.e),
            (false, 0x84) => self.add_a(self.registers.h),
            (false, 0x85) => self.add_a(self.registers.l),
            (false, 0xC3) => self.jump(true),
            (false, 0xC2) => self.jump(!self.registers.f.zero),
            (false, 0xD2) => self.jump(!self.registers.f.carry),
            (false, 0xCA) => self.jump(self.registers.f.zero),
            (false, 0xDA) => self.jump(self.registers.f.carry),
            (false, 0x01) => { self.registers.set_bc(self.read_addr()); self.registers.pc + 3 },
            (false, 0x11) => { self.registers.set_de(self.read_addr()); self.registers.pc + 3 },
            (false, 0x21) => { self.registers.set_hl(self.read_addr()); self.registers.pc + 3 },
            (false, 0x31) => { self.registers.sp =   self.read_addr();  self.registers.pc + 3 },
            (false, 0x08) => { *&mut self.memory.write_word(self.read_addr(), self.registers.sp);  self.registers.pc + 3 },
            _ => panic!("Unkown instruction found for: {instruction:#02x}, prefixed: {prefixed}")
        }
    }

    fn read_addr(&self) -> u16 {
        ((self.memory[self.registers.pc + 2] as u16) << 8) |
        (self.memory[self.registers.pc + 1] as u16)
    }

    fn add(&mut self, a: u8, b: u8) -> u8 {
        let (new_value, did_overflow) = a.overflowing_add(b);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (a & 0xF) + (b & 0xF) > 0xF;
        new_value
    }
    fn add_a(&mut self, value: u8) -> u16 {
        self.registers.a = self.add(self.registers.a, value);
        self.registers.pc + 1
    }

    fn jump(&self, condition: bool) -> u16 {
        if condition { self.read_addr() }
        else { self.registers.pc + 3 }
    }
}