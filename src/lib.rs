use crate::error::TmiError;
use crate::operation::*;
use std::collections::VecDeque;

pub mod cli;
mod error;
pub mod memory;
mod operation;

pub fn parse(file: &mut Vec<u8>, is_loop: bool) -> Result<VecDeque<Operation>, TmiError> {
    let mut ops: VecDeque<Operation> = VecDeque::new();

    loop {
        if file.is_empty() {
            return if is_loop {
                Err(TmiError::UnmatchedLoopOpen)
            } else {
                Ok(ops)
            }
        }
        let cmd = file.remove(0);
        match cmd {
            b'>' => ops.push_back(Operation::ShiftR),
            b'<' => ops.push_back(Operation::ShiftL),
            b'+' => ops.push_back(Operation::Inc),
            b'-' => ops.push_back(Operation::Dec),
            b'.' => ops.push_back(Operation::Access),
            b',' => ops.push_back(Operation::Set),
            b'[' => ops.push_back(Operation::Loop(parse(file, true)?)),
            b']' => {
                return if is_loop {
                    Ok(ops)
                } else {
                    Err(TmiError::UnmatchedLoopClose)
                }
            },
            _ => (),
        }
    }
}