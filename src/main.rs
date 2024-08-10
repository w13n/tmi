use clap::Parser;
use std::collections::VecDeque;
use std::fs;

use tmi::memory::{Mem, Memory};
use tmi::{cli, parse};
use tmi::cli::Cells;

fn main() {
    let args = cli::CArgs::parse();
    let mut contents = fs::read(args.file).unwrap();
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

    let mut input: VecDeque<u8> = VecDeque::from(Vec::from(args.input.unwrap_or_default()));

    eprintln!("parsing file");

    let ops = parse(&mut contents, false).unwrap();

    eprintln!("running file");

    for op in ops {
        op.execute(&mut *memory, &mut input).unwrap();
    }

    println!();
}
