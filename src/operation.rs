use std::collections::VecDeque;
use num::ToPrimitive;
use crate::error::TmiError;
use crate::memory::Memory;

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
    pub fn execute(&self, mem: &mut Box<&mut dyn Memory>, input: &mut VecDeque<u8>) -> Result<(), TmiError> {
        match self {
            Operation::Access => { Ok(print!("{}", mem.access()? as char)) }
            Operation::Set => { Ok(mem.set(input.pop_front().ok_or(TmiError {})?)) }
            Operation::ShiftL => { mem.shiftl() }
            Operation::ShiftR => { mem.shiftr() }
            Operation::Inc => { Ok(mem.inc()) }
            Operation::Dec => { Ok(mem.dec()) }
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