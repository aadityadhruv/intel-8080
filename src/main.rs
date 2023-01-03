mod debug;

use crate::debug::dassm;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Aaditya Dhruv", version = "0.1.0", about="", long_about = None)]
struct Args {
    /// Set this flag to run the ROM under debug mode 
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug : u8,
    /// The path to the ROM, e.g. /path/to/rom
    rom : String,
}


fn main() {
    let args = Args::parse();
    let rom = args.rom;
    let debug = args.debug > 0;
    println!("Debug mode is {}, loading ROM {}", if debug {"ON"} else {"OFF"}, rom);
    let mut dassm = dassm::IntelDebug::new();
    if debug {
        dassm.read_rom(&rom);
        dassm.dump_rom();
    }
}
