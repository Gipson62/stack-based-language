pub struct Memory {
    mem: Vec<u8>,
    blocks: Vec<Block>
}

pub struct Block {
    start: usize,
    end: usize,
    id: u64
}

impl Memory {
    pub fn get(&self, id: u64) -> Option<&[u8]> {
        for b in &self.blocks {
            if b.id == id {
                return Some(&self.mem[b.start..b.end])
            }
        }
        None
    }
    pub fn alloc(&self, size: usize) -> u64 {
        
    }
}