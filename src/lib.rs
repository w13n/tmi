use crate::cli::{CArgs, Cells};
use crate::error::TmiError;
use crate::memory::{Mem, Memory};
use crate::operation::*;
use std::collections::VecDeque;
use std::fs;

pub mod cli;
mod error;
mod memory;
mod operation;

pub fn parse(file: &mut Vec<u8>, is_loop: bool) -> Result<VecDeque<Operation>, TmiError> {
    let mut ops: VecDeque<Operation> = VecDeque::new();

    loop {
        if file.is_empty() {
            return if is_loop {
                Err(TmiError::UnmatchedLoopOpen)
            } else {
                Ok(ops)
            };
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
            }
            _ => (),
        }
    }
}

pub fn run(args: CArgs) -> Result<(), TmiError> {
    if args.debug {
        eprintln!("reading program")
    }

    let mut contents = fs::read(args.file).map_err(|_| TmiError::FileAccessError)?;

    if args.debug {
        eprintln!("creating memory")
    }

    let mut memory: Box<dyn Memory> = match args.cells {
        Cells::U8 => Box::new(Mem::<u8>::new(args.memory)),
        Cells::I8 => Box::new(Mem::<i8>::new(args.memory)),
        Cells::U16 => Box::new(Mem::<u16>::new(args.memory)),
        Cells::I16 => Box::new(Mem::<i16>::new(args.memory)),
        Cells::U32 => Box::new(Mem::<u32>::new(args.memory)),
        Cells::I32 => Box::new(Mem::<i32>::new(args.memory)),
        Cells::U64 => Box::new(Mem::<u64>::new(args.memory)),
        Cells::I64 => Box::new(Mem::<i64>::new(args.memory)),
        Cells::U128 => Box::new(Mem::<u128>::new(args.memory)),
        Cells::I128 => Box::new(Mem::<i128>::new(args.memory)),
    };

    if args.debug {
        eprintln!("building input")
    }

    let mut input: VecDeque<u8> = VecDeque::from(Vec::from(args.input.unwrap_or_default()));

    if args.debug {
        eprintln!("parsing program")
    }

    let ops = parse(&mut contents, false)?;

    if args.debug {
        eprintln!("running program")
    }

    for op in ops {
        op.execute(&mut *memory, &mut input)?;
    }
    Ok(())
}
