use crate::error::TmiError;
use crate::memory::{Cell, Memory};

pub(crate) struct FinMemory<T: Cell> {
    memory: Vec<T>,
    pos: usize,
    size: usize,
}

impl<T: Cell> FinMemory<T> {
    pub fn new(size: usize) -> FinMemory<T> {
        FinMemory {
            memory: vec![T::from_u8(0).unwrap(); size],
            pos: 0usize,
            size,
        }
    }
}

impl<T: Cell> std::fmt::Display for FinMemory<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.memory {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

impl<T: Cell> Memory for FinMemory<T> {
    fn access(&self) -> Result<u8, TmiError> {
        self.memory[self.pos]
            .to_u8()
            .ok_or(TmiError::MemoryAccessU8ConversionError)
    }

    fn set(&mut self, val: u8) {
        self.memory[self.pos] = T::from_u8(val).unwrap();
    }
    fn shiftl(&mut self) -> Result<(), TmiError> {
        if self.pos == 0 {
            return Err(TmiError::MemoryBoundsExceeded);
        }
        self.pos -= 1;
        Ok(())
    }
    fn shiftr(&mut self) -> Result<(), TmiError> {
        if self.pos >= self.size - 1 {
            return Err(TmiError::MemoryBoundsExceeded);
        }
        self.pos += 1;
        Ok(())
    }
    fn inc(&mut self) {
        self.memory[self.pos] = self.memory[self.pos].wrapping_add(&T::from_u8(1).unwrap());
    }

    fn dec(&mut self) {
        self.memory[self.pos] = self.memory[self.pos].wrapping_sub(&T::from_u8(1).unwrap());
    }
}
