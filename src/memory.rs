pub type Address = u32;

#[derive(Debug)]
pub enum BusCode {
    LoadAccessFault,
    StoreAccessFault,
}

type MemoryResult<T> = Result<T, BusCode>;

// Memory access is a trait.
pub trait Mem {
    fn read8(&self, address: Address) -> MemoryResult<u8>;
    fn read16(&self, address: Address) -> MemoryResult<u16>;
    fn read32(&self, address: Address) -> MemoryResult<u32>;
    fn write8(&mut self, address: Address, byte: u8) -> MemoryResult<()>;
    fn write16(&mut self, address: Address, half_word: u16) -> MemoryResult<()>;
    fn write32(&mut self, address: Address, word: u32) -> MemoryResult<()>;
}

// A very basic implementation of memory. The first 16K is ROM. The next 16K is RAM. Then there's a tty status register
// and a tty data register.
const MEMBASE: Address = 0;
const MEMSIZE: Address = 0x8000;
const RAMBASE: Address = 0x4000;
const RAMSIZE: Address = 0x4000;
const TTY_STATUS: Address = MEMBASE + MEMSIZE;
const TTY_DATA: Address = TTY_STATUS + 1;

pub struct BasicMem {
    mem: [u8; MEMSIZE as usize],
}

impl BasicMem {
    pub fn new() -> Self {
        BasicMem {
            mem: [0; MEMSIZE as usize],
        }
    }
}

pub trait Loader {
    fn write_bytes(&mut self, start: Address, bytes: &[u8]) -> MemoryResult<()>;
}

impl Loader for BasicMem {
    fn write_bytes(&mut self, start: Address, bytes: &[u8]) -> MemoryResult<()> {
        let start = start as usize;
        let end = start + bytes.len();
        if start >= MEMBASE as usize && end <= (MEMBASE + MEMSIZE) as usize {
            let start = start - MEMBASE as usize;
            let end = end - MEMBASE as usize;
            let dst = &mut self.mem[start..end];
            dst.copy_from_slice(bytes);
            return Ok(());
        }
        Err(BusCode::StoreAccessFault)
    }
}

impl Mem for BasicMem {
    fn read8(&self, address: Address) -> MemoryResult<u8> {
        if address >= MEMBASE && address < MEMBASE + MEMSIZE {
            Ok(self.mem[(address - MEMBASE) as usize])
        } else if address == TTY_STATUS {
            Ok(1)
        } else {
            Err(BusCode::LoadAccessFault)
        }
    }

    fn read16(&self, address: Address) -> MemoryResult<u16> {
        if address >= MEMBASE && address < MEMBASE + MEMSIZE - 1 {
            let addr = (address - MEMBASE) as usize;
            if let Ok(slice) = &self.mem[addr..addr + 2].try_into() {
                let result = u16::from_le_bytes(*slice);
                return Ok(result);
            }
        }
        Err(BusCode::LoadAccessFault)
    }

    fn read32(&self, address: Address) -> MemoryResult<u32> {
        if address >= MEMBASE && address < MEMBASE + MEMSIZE - 3 {
            let addr = (address - MEMBASE) as usize;
            if let Ok(slice) = &self.mem[addr..addr + 4].try_into() {
                let result = u32::from_le_bytes(*slice);
                return Ok(result);
            }
        }
        Err(BusCode::LoadAccessFault)
    }

    fn write8(&mut self, address: Address, byte: u8) -> MemoryResult<()> {
        if address >= RAMBASE && address < RAMBASE + RAMSIZE {
            let addr = (address - MEMBASE) as usize;
            self.mem[addr] = byte;
            Ok(())
        } else if address == TTY_DATA {
            print!("{}", byte as char);
            Ok(())
        } else {
            Err(BusCode::StoreAccessFault)
        }
    }

    fn write16(&mut self, address: Address, half_word: u16) -> MemoryResult<()> {
        if address >= MEMBASE && address < MEMBASE + MEMSIZE - 1 {
            let addr = (address - MEMBASE) as usize;
            let bytes: [u8; 2] = half_word.to_le_bytes();
            self.mem[addr + 0] = bytes[0];
            self.mem[addr + 1] = bytes[1];
            return Ok(());
        }
        Err(BusCode::StoreAccessFault)
    }

    fn write32(&mut self, address: Address, word: u32) -> MemoryResult<()> {
        if address >= MEMBASE && address < MEMBASE + MEMSIZE - 3 {
            let addr = (address - MEMBASE) as usize;
            let bytes: [u8; 4] = word.to_le_bytes();
            self.mem[addr + 0] = bytes[0];
            self.mem[addr + 1] = bytes[1];
            self.mem[addr + 2] = bytes[2];
            self.mem[addr + 3] = bytes[3];
            return Ok(());
        }
        Err(BusCode::StoreAccessFault)
    }
}

#[cfg(test)]
mod tests {
    use super::{BasicMem, Mem, TTY_DATA};

    #[test]
    fn test_the_tty_is_writable() {
        let mut mem = BasicMem::new();
        for c in b"Hello, world!\n" {
            mem.write8(TTY_DATA, *c);
        }
    }
}
