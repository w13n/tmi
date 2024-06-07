use crate::error::TmiError;
use num::traits::{WrappingAdd, WrappingSub};
use num::{FromPrimitive, Integer, ToPrimitive};

pub trait Memory: std::fmt::Display {
    fn access(&self) -> Result<u8, TmiError>;
    fn set(&mut self, val: u8);
    fn shiftl(&mut self) -> Result<(), TmiError>;
    fn shiftr(&mut self) -> Result<(), TmiError>;
    fn inc(&mut self);
    fn dec(&mut self);
}

pub struct InfMemory<
    T: ToPrimitive + FromPrimitive + WrappingAdd + WrappingSub + Integer + std::fmt::Display + Clone,
> {
    memory: Vec<T>,
    pos: usize,
    len: usize,
}

impl InfMemory<u8> {
    pub fn new() -> InfMemory<u8> {
        InfMemory {
            memory: vec![0],
            pos: 0usize,
            len: 0usize,
        }
    }
}

impl<
        T: ToPrimitive
            + FromPrimitive
            + WrappingAdd
            + WrappingSub
            + Integer
            + std::fmt::Display
            + Clone,
    > std::fmt::Display for InfMemory<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.memory {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

impl<
        T: ToPrimitive
            + FromPrimitive
            + WrappingAdd
            + WrappingSub
            + Integer
            + std::fmt::Display
            + Clone,
    > Memory for InfMemory<T>
{
    fn access(&self) -> Result<u8, TmiError> {
        self.memory
            .get(self.pos)
            .unwrap()
            .to_u8()
            .ok_or(TmiError::MemoryAccessU8ConversionError)
    }

    fn set(&mut self, val: u8) {
        self.memory[self.pos] = T::from_u8(val).unwrap();
    }
    fn shiftl(&mut self) -> Result<(), TmiError> {
        self.pos -= 1;
        Ok(())
    }
    fn shiftr(&mut self) -> Result<(), TmiError> {
        self.pos += 1;
        if self.len <= self.pos {
            self.memory.push(T::from_u8(0).unwrap());
            self.len += 1;
        }
        Ok(())
    }
    fn inc(&mut self) {
        self.memory[self.pos] = self.memory[self.pos].wrapping_add(&T::from_u8(1).unwrap());
    }

    fn dec(&mut self) {
        self.memory[self.pos] = self.memory[self.pos].wrapping_sub(&T::from_u8(1).unwrap());
    }
}
