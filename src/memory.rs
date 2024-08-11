use crate::error::TmiError;
use num::traits::{WrappingAdd, WrappingSub};
use num::{FromPrimitive, Integer, ToPrimitive};
use std::char::from_u32;

pub trait Memory {
    fn access(&self) -> Result<char, TmiError>;
    fn is_zero(&self) -> Result<bool, TmiError>;
    fn set(&mut self, val: u8) -> Result<(), TmiError>;
    fn shiftl(&mut self) -> Result<(), TmiError>;
    fn shiftr(&mut self) -> Result<(), TmiError>;
    fn inc(&mut self) -> Result<(), TmiError>;
    fn dec(&mut self) -> Result<(), TmiError>;
}

pub trait Cell:
    ToPrimitive + FromPrimitive + WrappingAdd + WrappingSub + Integer + std::fmt::Display + Clone
{
}
impl<
        T: ToPrimitive
            + FromPrimitive
            + WrappingAdd
            + WrappingSub
            + Integer
            + std::fmt::Display
            + Clone,
    > Cell for T
{
}

pub struct Mem<T: Cell> {
    cells: Vec<T>,
    pos: usize,
    len: usize,
    finite: bool,
}

impl<T: Cell> Mem<T> {
    pub fn new(size: Option<usize>) -> Mem<T> {
        if let Some(len) = size {
            Mem {
                cells: vec![T::from_u8(0).unwrap(); len],
                pos: 0usize,
                len,
                finite: true,
            }
        } else {
            Mem {
                cells: vec![T::from_u8(0).unwrap()],
                pos: 0usize,
                len: 0usize,
                finite: false,
            }
        }
    }
}

impl<T: Cell> Memory for Mem<T> {
    fn access(&self) -> Result<char, TmiError> {
        from_u32(
            self.cells[self.pos]
                .to_u32()
                .ok_or(TmiError::MemoryAccessU8ConversionError)?,
        )
        .ok_or(TmiError::MemoryAccessU8ConversionError)
    }

    fn is_zero(&self) -> Result<bool, TmiError> {
        Ok(self.cells[self.pos].is_zero())
    }

    fn set(&mut self, val: u8) -> Result<(), TmiError> {
        self.cells[self.pos] = T::from_u8(val).unwrap();
        Ok(())
    }
    fn shiftl(&mut self) -> Result<(), TmiError> {
        if self.pos == 0 {
            return Err(TmiError::MemoryLimitExceeded);
        }
        self.pos -= 1;
        Ok(())
    }
    fn shiftr(&mut self) -> Result<(), TmiError> {
        if self.len == self.pos {
            if self.finite {
                return Err(TmiError::MemoryLimitExceeded);
            } else {
                self.cells.push(T::from_u8(0).unwrap());
                self.len += 1;
            }
        }
        self.pos += 1;
        Ok(())
    }
    fn inc(&mut self) -> Result<(), TmiError> {
        self.cells[self.pos] = self.cells[self.pos].wrapping_add(&T::from_u8(1).unwrap());
        Ok(())
    }

    fn dec(&mut self) -> Result<(), TmiError> {
        self.cells[self.pos] = self.cells[self.pos].wrapping_sub(&T::from_u8(1).unwrap());
        Ok(())
    }
}
