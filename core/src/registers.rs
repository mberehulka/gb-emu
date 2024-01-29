#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub h: u8,
    pub l: u8,
    pub pc: u16, // program counter - address to the current instruction
    pub sc: u16  // stack counter - address to the end of the stack
}
impl Registers {
    pub fn get_bc(&self) -> u16 {
      (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
      self.b = (value >> 8) as u8;
      self.c = value as u8;
    }
}

#[derive(Default)]
pub struct FlagRegister {
    pub zero: bool, // set to 0 if the result of the operation is 0
    pub subtract: bool, // set to 1 if the operation is a subtraction, regardless of the result
    pub half_carry: bool, // half-carry, set to 1 if the operation is carrying or borrowing to bit 3
    pub carry: bool  // carry, set to 1 if the operation is carrying or borrowing to bit 7
}
impl Into<u8> for FlagRegister {
    fn into(self) -> u8 {
        match (self.zero, self.subtract, self.half_carry, self.carry) {
            (false, false, false, false) => 0b0000_0000,
            (false, false, false, true ) => 0b0001_0000,
            (false, false, true , false) => 0b0010_0000,
            (false, false, true , true ) => 0b0011_0000,
            (false, true , false, false) => 0b0100_0000,
            (false, true , false, true ) => 0b0101_0000,
            (false, true , true , false) => 0b0110_0000,
            (false, true , true , true ) => 0b0111_0000,
            (true , false, false, false) => 0b1000_0000,
            (true , false, false, true ) => 0b1001_0000,
            (true , false, true , false) => 0b1010_0000,
            (true , false, true , true ) => 0b1011_0000,
            (true , true , false, false) => 0b1100_0000,
            (true , true , false, true ) => 0b1101_0000,
            (true , true , true , false) => 0b1110_0000,
            (true , true , true , true ) => 0b1111_0000
        }
    }
}
impl From<u8> for FlagRegister {
    fn from(value: u8) -> Self {
        Self {
            zero: (value & 0b1000_0000) != 0,
            subtract:  (value & 0b0100_0000) != 0,
            half_carry:   (value & 0b0010_0000) != 0,
            carry:    (value & 0b0001_0000) != 0
        }
    }
}