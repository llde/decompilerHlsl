use clap::Clap;
use std::fs::File;
use  std::io;
use std::io::Read;

mod functions;
mod intermediate;
mod parser;
mod asm;

use crate::asm::FileAsm;

#[derive(Clap)]
#[clap(version = "1.0", author = "llde")]
struct Opts {
    #[clap(short = "i", long = "input")]
    input: String,
}

fn main() ->Result<(), io::Error> {
    let opts: Opts = Opts::parse();
    let mut file = File::open(opts.input)?;
    let mut interm = String::new();
    if file.read_to_string(&mut interm).is_err() {
        println!("Can't read file");
    }
    
    let asm = FileAsm::new(interm);
    let intermediate = asm.decompile_step();
    println!("{:?}", intermediate);
    return Ok(());
}
