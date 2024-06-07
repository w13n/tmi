use crate::error::TmiError;
use crate::operation::*;
use std::collections::VecDeque;

mod error;
pub mod memory;
mod operation;

pub fn parse(file: &mut Vec<u8>) -> Result<VecDeque<Operation>, TmiError> {
    let mut ops: VecDeque<Operation> = VecDeque::new();
    while !file.is_empty() {
        let cmd = file.remove(0);
        match cmd {
            b'>' => ops.push_back(Operation::ShiftR),
            b'<' => ops.push_back(Operation::ShiftL),
            b'+' => ops.push_back(Operation::Inc),
            b'-' => ops.push_back(Operation::Dec),
            b'.' => ops.push_back(Operation::Access),
            b',' => ops.push_back(Operation::Set),
            b'[' => ops.push_back(Operation::Loop(parse_loop(file)?)),
            b']' => return Err(TmiError::UnmatchedLoopClose),
            _ => (),
        }
    }
    Ok(ops)
}

fn parse_loop(file: &mut Vec<u8>) -> Result<VecDeque<Operation>, TmiError> {
    let mut ops: VecDeque<Operation> = VecDeque::new();
    let mut cmd = file.remove(0);
    while cmd != b']' {
        match cmd {
            b'>' => ops.push_back(Operation::ShiftR),
            b'<' => ops.push_back(Operation::ShiftL),
            b'+' => ops.push_back(Operation::Inc),
            b'-' => ops.push_back(Operation::Dec),
            b'.' => ops.push_back(Operation::Access),
            b',' => ops.push_back(Operation::Set),
            b'[' => ops.push_back(Operation::Loop(parse_loop(file)?)),
            _ => (),
        };
        cmd = file.remove(0);
    }
    Ok(ops)
}
