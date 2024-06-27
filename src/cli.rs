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
    /// cell size in bits (signed or unsigned)
    #[arg(value_enum, short, long, default_value_t = Cells::U8)]
    pub cells: Cells,
}

#[derive(Clone, ValueEnum)]
pub enum Cells {
    U8,
    S8,
    U16,
    S16,
    U32,
    S32,
    U64,
    S64,
    U128,
    S128,
}
