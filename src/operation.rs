use crate::error::TmiError;
use crate::memory::Memory;
use std::collections::VecDeque;

pub enum Operation {
    Access,
    Set,
    ShiftL,
    ShiftR,
    Inc,
    Dec,
    Loop(VecDeque<Operation>),
}

impl Operation {
    pub fn execute(&self, mem: &mut dyn Memory, input: &mut VecDeque<u8>) -> Result<(), TmiError> {
        match self {
            Operation::Access => {
                print!("{}", mem.access()?);
                Ok(())
            }
            Operation::Set => mem.set(input.pop_front().unwrap_or_default()),
            Operation::ShiftL => mem.shiftl(),
            Operation::ShiftR => mem.shiftr(),
            Operation::Inc => mem.inc(),
            Operation::Dec => mem.dec(),
            Operation::Loop(ops) => {
                while !mem.is_zero()? {
                    for op in ops {
                        op.execute(mem, input)?
                    }
                }
                Ok(())
            }
        }
    }
}
