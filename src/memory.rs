pub mod fin;
pub mod inf;

use crate::error::TmiError;
use crate::memory::fin::FinMemory;
use crate::memory::inf::InfMemory;
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

trait Cell:
    ToPrimitive + FromPrimitive + WrappingAdd + WrappingSub + Integer + std::fmt::Display + Clone {}
impl<
        T: ToPrimitive
            + FromPrimitive
            + WrappingAdd
            + WrappingSub
            + Integer
            + std::fmt::Display
            + Clone,
    > Cell for T {}

pub fn create_cells<T: Cell + 'static>(cells: Option<usize>) -> Box<dyn Memory> {
    if let Some(memory) = cells {
        Box::new(FinMemory::<T>::new(memory))
    } else {
        Box::new(InfMemory::<T>::new())
    }
}
