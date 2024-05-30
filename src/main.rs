use std::env;
use std::fs;

use tmi::*;

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap();
    let mut input = args.next().unwrap().as_bytes().into();
    let mut contents = fs::read(path).unwrap();
    let mut memory: memory::InfMemory<i8> = memory::InfMemory::new();
    let mut output = Vec::new();


    println!("parsing file");

    let ops = tmi::parse(&mut contents);

    println!("running file");

    for op in ops {
        op.op(&mut output, &mut input, &mut memory);
    }

    println!("mem: {}", memory);
    println!("{}", String::from_utf8(output).unwrap());
}
