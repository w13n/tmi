use num::Integer;
use num::traits::{WrappingAdd, WrappingSub};
use crate::error::TmiError;

pub trait Memory: std::fmt::Display {
    fn access(&self) -> u8;
    fn set(&mut self, val: u8);
    fn shiftl(&mut self) -> Result<(), TmiError>;
    fn shiftr(&mut self);
    fn inc(&mut self);
    fn dec(&mut self);
}

pub struct InfMemory<T: Integer + WrappingAdd + WrappingSub> {
    memory: Vec<T>,
    pos: usize,
}

impl InfMemory<i8> {
    pub fn new() -> InfMemory<i8> {
        InfMemory {
            memory: vec![0],
            pos: 0usize,
        }
    }
}

impl std::fmt::Display for InfMemory<i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.memory {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

impl Memory for InfMemory<i8> {
    fn access(&self) -> u8 {
        *self.memory.get(self.pos).unwrap() as u8
    }

    fn set(&mut self, val: u8) {
        self.memory[self.pos] == val as i8;
    }
    fn shiftl(&mut self) -> Result<(), TmiError> {
        if self.pos < 0 {
            return Err(TmiError{});
        }
        self.pos -= 1;
        Ok(())
    }
    fn shiftr(&mut self) {
        self.pos += 1;
        if self.memory.len() <= self.pos {
            self.memory.push(0);
        }
    }
    fn inc(&mut self) {
        self.memory[self.pos] = self.memory[self.pos].wrapping_add(1);
    }
    fn dec(&mut self) {
        self.memory[self.pos] = self.memory[self.pos].wrapping_sub(1);
    }
}