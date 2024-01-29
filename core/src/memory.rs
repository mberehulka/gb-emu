use std::ops::Index;

#[derive(Default)]
pub struct Memory {
    pub bytes: Vec<u8>
}
impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output {
        &self.bytes[address as usize]
    }
}