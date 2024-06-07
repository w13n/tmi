use std::collections::VecDeque;
use crate::memory::*;

pub trait Operation {
    fn op(&self, output: &mut Vec<u8>, input: &mut Vec<u8>, mem: &mut InfMemory<i8>);
}

pub struct Inc {}

impl Inc {
    pub fn new() -> Inc {
        Inc {}
    }
}

impl Operation for Inc {
    fn op(&self, _: &mut Vec<u8>, _: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        mem.inc();
    }
}

pub struct Dec {}

impl Dec {
    pub fn new() -> Dec {
        Dec {}
    }
}

impl Operation for Dec {
    fn op(&self, _: &mut Vec<u8>, _: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        mem.dec();
    }
}

pub struct ShiftL {}

impl ShiftL {
    pub fn new() -> ShiftL {
        ShiftL {}
    }
}

impl Operation for ShiftL {
    fn op(&self, _: &mut Vec<u8>, _: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        mem.shiftl();
    }
}

pub struct ShiftR {}

impl ShiftR {
    pub fn new() -> ShiftR {
        ShiftR {}
    }
}

impl Operation for ShiftR {
    fn op(&self, _: &mut Vec<u8>, _: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        mem.shiftr();
    }
}

pub struct Loop {
    ops: VecDeque<Box<dyn Operation>>,
}

impl Loop {
    pub fn new(ops: VecDeque<Box<dyn Operation>>) -> Loop {
        Loop { ops }
    }
}

impl Operation for Loop {
    fn op(&self, output: &mut Vec<u8>, input: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        while mem.access() != 0 {
            for operator in &self.ops {
                operator.op(output, input, mem);
            }
        }
    }
}

pub struct Input {}

impl Input {
    fn new() -> Input {
        Input {}
    }
}

impl Operation for Input {
    fn op(&self, _: &mut Vec<u8>, input: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        mem.set(input.remove(0));
    }
}

pub struct Output {}

impl Output {
    pub fn new() -> Output {
        Output {}
    }
}

impl Operation for Output {
    fn op(&self, output: &mut Vec<u8>, _: &mut Vec<u8>, mem: &mut InfMemory<i8>) {
        output.push(mem.access());
    }
}