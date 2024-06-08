use std::env;
use std::fs;

use tmi::*;



fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap();
    let mut input = args.next().unwrap().as_bytes().to_vec().into();
    let mut contents = fs::read(path).unwrap();
    let mut memory: memory::fin::FinMemory<i128> = memory::fin::FinMemory::new(300);

    println!("parsing file");

    let ops = parse(&mut contents).unwrap();

    println!("running file");

    for op in ops {
        op.execute(&mut memory, &mut input).unwrap();
    }

    println!();

    println!("mem: {}", memory);
}
