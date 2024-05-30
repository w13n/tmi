use crate::operation::*;

mod error;
mod operation;
pub mod memory;

pub fn parse(file: &mut Vec<u8>) -> Vec<Box<dyn Operation>> {
    let mut ops: Vec<Box<dyn Operation>> = Vec::new();
    while !file.is_empty() {
        let cmd = file.remove(0);
        match cmd {
            b'>' => ops.push(Box::new(ShiftR::new())),
            b'<' => ops.push(Box::new(ShiftL::new())),
            b'+' => ops.push(Box::new(Inc::new())),
            b'-' => ops.push(Box::new(Dec::new())),
            b'.' => ops.push(Box::new(Output::new())),
            b',' => ops.push(Box::new(Output::new())),
            b'[' => ops.push(Box::new(parse_loop(file))),
            b']' => panic!("too many ]"),
            _ => (),
        }
    }
    ops
}

fn parse_loop(file: &mut Vec<u8>) -> Loop {
    let mut ops: Vec<Box<dyn Operation>> = Vec::new();
    let mut cmd = file.remove(0);
    while cmd != b']' {
        match cmd {
            b'>' => ops.push(Box::new(ShiftR::new())),
            b'<' => ops.push(Box::new(ShiftL::new())),
            b'+' => ops.push(Box::new(Inc::new())),
            b'-' => ops.push(Box::new(Dec::new())),
            b'.' => ops.push(Box::new(Output::new())),
            b',' => ops.push(Box::new(Output::new())),
            b'[' => ops.push(Box::new(parse_loop(file))),
            _ => (),
        };
        cmd = file.remove(0);
    }
    Loop::new(ops)
}
