use clap::Parser;
use std::collections::VecDeque;
use std::fs;

use tmi::cli::Cells;
use tmi::memory::Memory;
use tmi::*;

fn main() {
    let args = cli::CArgs::parse();
    let mut contents = fs::read(args.file).unwrap();
    let mut memory: Box<dyn Memory> = match args.cells {
        Cells::U8 => memory::create_cells::<u8>(args.memory),
        Cells::S8 => memory::create_cells::<i8>(args.memory),
        Cells::U16 => memory::create_cells::<u16>(args.memory),
        Cells::S16 => memory::create_cells::<i16>(args.memory),
        Cells::U32 => memory::create_cells::<u32>(args.memory),
        Cells::S32 => memory::create_cells::<i32>(args.memory),
        Cells::U64 => memory::create_cells::<u64>(args.memory),
        Cells::S64 => memory::create_cells::<i64>(args.memory),
        Cells::U128 => memory::create_cells::<u128>(args.memory),
        Cells::S128 => memory::create_cells::<i128>(args.memory),
    };

    let mut input: VecDeque<u8> = VecDeque::from(Vec::from(args.input.unwrap_or_default()));

    println!("parsing file");

    let ops = parse(&mut contents).unwrap();

    println!("running file");

    for op in ops {
        op.execute(&mut *memory, &mut input).unwrap();
    }

    println!();
}
