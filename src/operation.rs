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
                print!("{}", mem.access()? as char);
                Ok(())
            }
            Operation::Set => {
                mem.set(input.pop_front().ok_or(TmiError {})?);
                Ok(())
            }
            Operation::ShiftL => mem.shiftl(),
            Operation::ShiftR => mem.shiftr(),
            Operation::Inc => {
                mem.inc();
                Ok(())
            }
            Operation::Dec => {
                mem.dec();
                Ok(())
            }
            Operation::Loop(ops) => {
                while mem.access()? != 0 {
                    for op in ops {
                        op.execute(mem, input)?
                    }
                }
                Ok(())
            }
        }
    }
}
