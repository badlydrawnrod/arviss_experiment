pub type Address = u32;

pub type MemoryResult<T> = Result<T, Address>;

/// Memory access as a trait.
pub trait Mem {
    /// Reads a byte from memory.
    fn read8(&self, address: Address) -> MemoryResult<u8>;

    /// Reads a 16-bit half-word from memory.
    fn read16(&self, address: Address) -> MemoryResult<u16>;

    /// Reads a 32-bit word from memory.
    fn read32(&self, address: Address) -> MemoryResult<u32>;

    /// Writes a byte to memory.
    fn write8(&mut self, address: Address, byte: u8) -> MemoryResult<()>;

    /// Writes a 16-bit half word to memory.
    fn write16(&mut self, address: Address, half_word: u16) -> MemoryResult<()>;
    
    /// Writes a 32-bit word to memory.
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

/// A very simple memory implementation.
/// 
/// A very basic implementation of memory. The first 16K is ROM. The next 16K is RAM. Then there's a tty status register
/// and a tty data register.
///
/// The address space has the following layout.
///
/// |  Start |    End | Usage |
/// |--------|--------|-------|
/// | 0x0000 | 0x3fff | ROM   |
/// | 0x4000 | 0x7fff | RAM   |
/// | 0x8000 | 0x8001 | I/O   |
///
/// Where:
///
/// | Address | Usage | Access |
/// |---------|-------|--------|
/// |  0x8000 | TTY status | R/O |
/// |  0x8001 | TTY data   | W/O |
///
pub struct BasicMem {
    mem: [u8; MEMSIZE as usize],
}

impl Default for BasicMem {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicMem {
    pub fn new() -> Self {
        BasicMem {
            mem: [0; MEMSIZE as usize],
        }
    }
}

/// Loads data into memory.
pub trait Loader {
    /// Loads a slice of bytes into memory starting at the given address.
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
        Err(start as u32)
    }
}

impl Mem for BasicMem {
    #[inline]
    fn read8(&self, address: Address) -> MemoryResult<u8> {
        if (MEMBASE..MEMBASE + MEMSIZE).contains(&address) {
            Ok(self.mem[(address - MEMBASE) as usize])
        } else if address == TTY_STATUS {
            Ok(1)
        } else {
            Err(address)
        }
    }

    #[inline]
    fn read16(&self, address: Address) -> MemoryResult<u16> {
        if (MEMBASE..MEMBASE + MEMSIZE - 1).contains(&address) {
            let addr = (address - MEMBASE) as usize;
            if let Ok(slice) = &self.mem[addr..addr + 2].try_into() {
                let result = u16::from_le_bytes(*slice);
                return Ok(result);
            }
        }
        Err(address)
    }

    #[inline]
    fn read32(&self, address: Address) -> MemoryResult<u32> {
        if (MEMBASE..MEMBASE + MEMSIZE - 3).contains(&address) {
            let addr = (address - MEMBASE) as usize;
            if let Ok(slice) = &self.mem[addr..addr + 4].try_into() {
                let result = u32::from_le_bytes(*slice);
                return Ok(result);
            }
        }
        Err(address)
    }

    #[inline]
    fn write8(&mut self, address: Address, byte: u8) -> MemoryResult<()> {
        if (RAMBASE..RAMBASE + RAMSIZE).contains(&address) {
            let addr = (address - MEMBASE) as usize;
            self.mem[addr] = byte;
            Ok(())
        } else if address == TTY_DATA {
            print!("{}", byte as char);
            Ok(())
        } else {
            Err(address)
        }
    }

    #[inline]
    fn write16(&mut self, address: Address, half_word: u16) -> MemoryResult<()> {
        if (RAMBASE..RAMBASE + RAMSIZE - 1).contains(&address) {
            let addr = (address - MEMBASE) as usize;
            let bytes: [u8; 2] = half_word.to_le_bytes();
            self.mem[addr] = bytes[0];
            self.mem[addr + 1] = bytes[1];
            return Ok(());
        }
        Err(address)
    }

    #[inline]
    fn write32(&mut self, address: Address, word: u32) -> MemoryResult<()> {
        if (RAMBASE..RAMBASE + RAMSIZE - 3).contains(&address) {
            let addr = (address - MEMBASE) as usize;
            let bytes: [u8; 4] = word.to_le_bytes();
            self.mem[addr] = bytes[0];
            self.mem[addr + 1] = bytes[1];
            self.mem[addr + 2] = bytes[2];
            self.mem[addr + 3] = bytes[3];
            return Ok(());
        }
        Err(address)
    }
}

#[cfg(test)]
mod tests {
    use super::{BasicMem, Mem, TTY_DATA};

    #[test]
    fn test_the_tty_is_writable() {
        let mut mem = BasicMem::new();
        for c in b"Hello, world!\n" {
            mem.write8(TTY_DATA, *c).expect("Failed to write to TTY")
        }
    }
}
