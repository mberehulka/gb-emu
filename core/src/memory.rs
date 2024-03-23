pub struct MemoryDevice {
    pub start: u16,
    pub end: u16
}
impl MemoryDevice {
    pub const fn size(&self) -> usize {
        (self.end - self.start) as usize
    }
}

pub const ROM:  MemoryDevice = MemoryDevice { start: 0x0000, end: 0x7FFF }; // From cartridge, usually a fixed bank
pub const VRAM: MemoryDevice = MemoryDevice { start: 0x8000, end: 0x9FFF }; // Video RAM, where graphics are stored and arranged
pub const SRAM: MemoryDevice = MemoryDevice { start: 0xA000, end: 0xBFFF }; // Save RAM, optionally supplied by the cartridge to save data to
pub const WRAM: MemoryDevice = MemoryDevice { start: 0xC000, end: 0xDFFF }; // Work RAM, general-purpose RAM for the game to store things in
pub const OAM:  MemoryDevice = MemoryDevice { start: 0xFE00, end: 0xFE9F }; // Object Attribute Memory, where “objects” are stored
pub const IO:   MemoryDevice = MemoryDevice { start: 0xFF00, end: 0xFF7F }; // I/O - Neither ROM nor RAM, but this is where you control the console
pub const HRAM: MemoryDevice = MemoryDevice { start: 0xFF80, end: 0xFFFE }; // High RAM, a tiny bit of general-purpose RAM which can be accessed faster
pub const IE:   MemoryDevice = MemoryDevice { start: 0xFFFF, end: 0xFFFF }; // A lone I/O byte that’s separated from the rest for some reason

pub struct Memory {
    pub rom: [u8; ROM.size()],
    pub hram: [u8; HRAM.size()],
}

impl Memory {
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.wb(address, (value & 0xFF) as u8);
        self.wb(address + 1, (value >> 8) as u8);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            rom: [0; ROM.size()],
            hram: [0; HRAM.size()]
        }
    }
}

impl std::ops::Index<u16> for Memory {
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output {
        &self.0[address as usize]
    }
}
impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        &mut self.0[address as usize]
    }
}