//! Core traits used to implement memory.

/// A memory address.
pub type Address = u32;

/// The result of a memory operation. The error variant returns the address that caused the error.
pub type MemoryResult<T> = Result<T, Address>;

/// Trait for memory access.
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

/// Loads data into memory.
pub trait Loader {
    /// Loads a slice of bytes into memory starting at the given address.
    fn write_bytes(&mut self, start: Address, bytes: &[u8]) -> MemoryResult<()>;
}
