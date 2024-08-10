use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CArgs {
    /// source code file of the bf program to run
    #[arg(short, long)]
    pub file: PathBuf,
    /// input to the program
    #[arg(short, long)]
    pub input: Option<String>,
    /// number of memory cells to use (leave blank for dynamic sized memory)
    #[arg(short, long)]
    pub memory: Option<usize>,
    /// cell size in bits (integer or unsigned)
    #[arg(value_enum, short, long, default_value_t = Cells::U8)]
    pub cells: Cells,
}

#[derive(Clone, ValueEnum)]
pub enum Cells {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
}
